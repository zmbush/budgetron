use budgetronlib::fintime::{Date, Timeframe};
use loading::Transaction;
use reporting::Reporter;
use serde_json::{self, Value};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;

pub struct ByTimeframe<'a, T>
    where T: 'a + Reporter
{
    inner: &'a T,
    timeframe: Timeframe,
}

impl<'a, T> ByTimeframe<'a, T>
    where T: 'a + Reporter
{
    pub fn new(inner: &'a T, timeframe: Timeframe) -> Self {
        ByTimeframe { inner, timeframe }
    }
}

#[derive(Debug, Serialize)]
pub struct ByTimeframeReport<T> {
    timeframe: Timeframe,
    by_timeframe: BTreeMap<Date, T>,
}

impl<T> ByTimeframeReport<T>
    where T: fmt::Display
{
    pub fn print(&self) {
        println!("{}", self)
    }
}

impl<T> fmt::Display for ByTimeframeReport<T>
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (key, value) in &self.by_timeframe {
            writeln!(f,
                     "For the transactions in {}-{}",
                     key,
                     key + self.timeframe - Timeframe::Days(1))?;
            writeln!(f, "{}", value)?;
        }
        Ok(())
    }
}

impl<'a, T> Reporter for ByTimeframe<'a, T>
    where T: Reporter
{
    fn report<'b, I>(&self, transactions: I) -> Value
        where I: Iterator<Item = Cow<'b, Transaction>>
    {
        let mut transactions: Vec<_> = transactions.collect();
        let mut date = transactions
            .get(0)
            .map(|t| t.date)
            .clone()
            .unwrap_or_else(|| Date::ymd(2000, 1, 1));

        match self.timeframe {
            Timeframe::Days(_) => {},
            Timeframe::Weeks(_) => date.align_to_week(),
            Timeframe::Months(_) => date.align_to_month(),
            Timeframe::Quarters(_) => date.align_to_quarter(),
            Timeframe::Years(_) => date.align_to_year(),
        }

        let mut by_timeframe = BTreeMap::new();
        while transactions.len() > 0 {
            let (current, remaining): (Vec<_>, Vec<_>) =
                transactions
                    .into_iter()
                    .partition(|t| t.date >= date && t.date < date + self.timeframe);
            transactions = remaining;
            let mut map = serde_json::map::Map::new();
            if let Some(v) = self.inner.key() {
                map.insert(v.to_owned(), self.inner.report(current.into_iter()));
            } else {
                match self.inner.report(current.into_iter()) {
                    Value::Object(o) => {
                        for (k, v) in o {
                            map.insert(k, v);
                        }
                    },
                    other => {
                        map.insert("by_timeframe".to_owned(), other);
                    },
                }
            }
            by_timeframe.insert(date, map);
            date += self.timeframe;
        }
        let mut retval = serde_json::to_value(by_timeframe).expect("shitballs");
        if let Some(mut obj) = retval.as_object_mut() {
            obj.insert("timeframe".to_owned(),
                       serde_json::to_value(self.timeframe).expect("shibble"));
        }
        retval
    }

    fn key(&self) -> Option<String> {
        Some(format!("by_timeframe{:?}", self.timeframe))
    }

    fn description(&self) -> Vec<String> {
        self.inner
            .description()
            .into_iter()
            .map(|d| format!("{} by timeframe `{}`", d, self.timeframe))
            .collect()
    }
}

use budgetronlib::fintime::{Date, Timeframe};
use loading::Transaction;
use reporting::Reporter;
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
    type OutputType = ByTimeframeReport<T::OutputType>;

    fn report<'b, I>(&self, transactions: I) -> ByTimeframeReport<T::OutputType>
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
            by_timeframe.insert(date, self.inner.report(current.into_iter()));
            date += self.timeframe;
        }
        ByTimeframeReport {
            by_timeframe,
            timeframe: self.timeframe,
        }
    }
}

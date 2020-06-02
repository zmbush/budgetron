// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    crate::{loading::Transaction, reporting::Reporter},
    budgetronlib::fintime::{Date, Timeframe},
    serde::Serialize,
    serde_json::{self, Value},
    std::{borrow::Cow, collections::BTreeMap, fmt},
};

pub struct ByTimeframe<'a, T>
where
    T: 'a + Reporter,
{
    inner: &'a T,
    timeframe: Timeframe,
}

impl<'a, T> ByTimeframe<'a, T>
where
    T: 'a + Reporter,
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
where
    T: fmt::Display,
{
    pub fn print(&self) {
        println!("{}", self)
    }
}

impl<T> fmt::Display for ByTimeframeReport<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (key, value) in &self.by_timeframe {
            writeln!(
                f,
                "For the transactions in {}-{}",
                key,
                key + self.timeframe - Timeframe::Days(1)
            )?;
            writeln!(f, "{}", value)?;
        }
        Ok(())
    }
}

impl<'a, T> Reporter for ByTimeframe<'a, T>
where
    T: Reporter,
{
    fn report<'b, I>(&self, transactions: I, end_date: Date) -> Value
    where
        I: Iterator<Item = Cow<'b, Transaction>>,
    {
        let mut transactions: Vec<_> = transactions.collect();
        let mut date = transactions
            .get(0)
            .map(|t| t.date)
            .unwrap_or_else(|| Date::ymd(2000, 1, 1));

        match self.timeframe {
            Timeframe::Days(_) => {}
            Timeframe::Weeks(_) => date.align_to_week(),
            Timeframe::Months(_) => date.align_to_month(),
            Timeframe::Quarters(_) => date.align_to_quarter(),
            Timeframe::Years(_) => date.align_to_year(),
        }

        let mut by_timeframe = BTreeMap::new();
        while !transactions.is_empty() {
            let (current, remaining): (Vec<_>, Vec<_>) = transactions
                .into_iter()
                .partition(|t| t.date >= date && t.date < date + self.timeframe);
            transactions = remaining;
            by_timeframe.insert(date, self.inner.report(current.into_iter(), end_date));
            date += self.timeframe;
        }

        serde_json::to_value(by_timeframe).expect("Unable to serialize by_timeframe")
    }

    fn key(&self) -> Option<String> {
        Some(
            self.timeframe
                .ly()
                .to_lowercase()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join("_"),
        )
    }
}

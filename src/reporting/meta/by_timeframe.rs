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
    std::{borrow::Cow, collections::BTreeMap},
};

pub struct ByTimeframe<'i, T> {
    inner: &'i T,
    timeframe: Timeframe,
}

impl<'i, T> ByTimeframe<'i, T> {
    pub fn new(inner: &'i T, timeframe: Timeframe) -> Self {
        ByTimeframe { inner, timeframe }
    }
}

impl<'i, T> ByTimeframe<'i, T>
where
    T: Reporter,
{
    pub fn report<'t>(
        &self,
        transactions: impl Iterator<Item = Cow<'t, Transaction>> + Clone,
    ) -> BTreeMap<Date, crate::reporting::data::ConcreteReport> {
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
            by_timeframe.insert(date, self.inner.report(current.into_iter()));
            date += self.timeframe;
        }

        by_timeframe
    }
}

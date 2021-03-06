// Copyright 2018 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    crate::{loading::Transaction, reporting::Reporter},
    budgetronlib::fintime::Date,
    serde_json::Value,
    std::borrow::Cow,
};

pub struct OnlyOwners<'a, T>
where
    T: 'a + Reporter,
{
    inner: &'a T,
    owners: Vec<String>,
}

impl<'a, T> OnlyOwners<'a, T>
where
    T: 'a + Reporter,
{
    pub fn new(inner: &'a T, owners: Vec<String>) -> Self {
        OnlyOwners { inner, owners }
    }
}

impl<'a, T> Reporter for OnlyOwners<'a, T>
where
    T: Reporter,
{
    fn report<'b, I>(&self, transactions: I, end_date: Date) -> Value
    where
        I: Iterator<Item = Cow<'b, Transaction>>,
    {
        let (transactions, _): (Vec<_>, Vec<_>) =
            transactions.partition(|t| self.owners.iter().any(|owner| t.person == *owner));
        self.inner.report(transactions.into_iter(), end_date)
    }

    fn key(&self) -> Option<String> {
        Some(format!("only_owners_{}", self.owners.join("_")))
    }
}

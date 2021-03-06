// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    crate::{
        loading::{Transaction, TransactionType},
        reporting::Reporter,
    },
    budgetronlib::fintime::Date,
    serde_json::Value,
    std::borrow::Cow,
};

pub struct OnlyType<'a, T>
where
    T: 'a + Reporter,
{
    inner: &'a T,
    t: TransactionType,
}

impl<'a, T> OnlyType<'a, T>
where
    T: 'a + Reporter,
{
    pub fn new(inner: &'a T, t: TransactionType) -> Self {
        OnlyType { inner, t }
    }
}

impl<'a, T> Reporter for OnlyType<'a, T>
where
    T: Reporter,
{
    fn report<'b, I>(&self, transactions: I, end_date: Date) -> Value
    where
        I: Iterator<Item = Cow<'b, Transaction>>,
    {
        let (transactions, _): (Vec<_>, Vec<_>) =
            transactions.partition(|t| t.transaction_type == self.t);

        self.inner.report(transactions.into_iter(), end_date)
    }

    fn key(&self) -> Option<String> {
        Some(format!("only_type_{:?}", self.t))
    }
}

// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use loading::Transaction;
use reporting::Reporter;
use serde_json::Value;
use std::borrow::Cow;

pub struct OnlyTags<'a, T>
where
    T: 'a + Reporter,
{
    inner: &'a T,
    tags:  Vec<String>,
}

impl<'a, T> OnlyTags<'a, T>
where
    T: 'a + Reporter,
{
    pub fn new(inner: &'a T, tags: Vec<String>) -> Self {
        OnlyTags { inner, tags }
    }
}

impl<'a, T> Reporter for OnlyTags<'a, T>
where
    T: Reporter,
{
    fn report<'b, I>(&self, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'b, Transaction>>,
    {
        let (transactions, _): (Vec<_>, Vec<_>) = transactions
            .into_iter()
            .partition(|t| self.tags.iter().any(|tag| t.tags.contains(tag)));

        self.inner.report(transactions.into_iter())
    }

    fn key(&self) -> Option<String> {
        Some(format!("only_tags_{}", self.tags.join("_")))
    }
}

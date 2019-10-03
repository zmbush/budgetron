// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::loading::{Transaction, TransactionType};
use crate::reporting::Reporter;
use serde_derive::Serialize;
use serde_json::{self, Value};
use std::borrow::Cow;
use std::fmt;

pub struct ByAccount<'a, T>
where
    T: 'a + Reporter,
{
    inner: &'a T,
    account: String,
}

impl<'a, T> ByAccount<'a, T>
where
    T: 'a + Reporter,
{
    pub fn new(inner: &'a T, account: String) -> Self {
        ByAccount { inner, account }
    }
}

#[derive(Debug, Serialize)]
pub struct ByAccountReport<T> {
    account: String,
    by_account: T,
}

impl<T> ByAccountReport<T>
where
    T: fmt::Display,
{
    pub fn print(&self) {
        println!("{}", self)
    }
}

impl<T> fmt::Display for ByAccountReport<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "For the account {}", self.account)?;
        writeln!(f, "{}", self.by_account)
    }
}

impl<'a, T> Reporter for ByAccount<'a, T>
where
    T: Reporter,
{
    fn report<'b, I>(&self, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'b, Transaction>>,
    {
        let (transactions, _): (Vec<_>, Vec<_>) = transactions
            .into_iter()
            .map(|t| {
                if let TransactionType::Transfer = t.transaction_type {
                    if t.account_name == self.account {
                        let mut t = t.into_owned();
                        t.transaction_type = TransactionType::Debit;
                        t.transfer_destination_account = None;
                        Cow::Owned(t)
                    } else if *t
                        .transfer_destination_account
                        .as_ref()
                        .expect("all transfers should have destinations")
                        == self.account
                    {
                        let mut t = t.into_owned();
                        t.transaction_type = TransactionType::Credit;
                        t.account_name = t.transfer_destination_account.take().unwrap();
                        Cow::Owned(t)
                    } else {
                        t
                    }
                } else {
                    t
                }
            })
            .partition(|t| t.account_name == self.account);

        let mut retval = serde_json::map::Map::new();
        retval.insert("account".to_owned(), Value::String(self.account.clone()));
        if let Some(v) = self.inner.key() {
            retval.insert(v.to_owned(), self.inner.report(transactions.into_iter()));
        } else {
            match self.inner.report(transactions.into_iter()) {
                Value::Object(o) => {
                    for (k, v) in o {
                        retval.insert(k, v);
                    }
                }
                other => {
                    retval.insert("by_account".to_owned(), other);
                }
            }
        }
        Value::Object(retval)
    }

    fn key(&self) -> Option<String> {
        Some(format!(
            "for_{}",
            self.account
                .to_lowercase()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join("_")
        ))
    }
}

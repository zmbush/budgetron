// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use loading::{Money, Transaction, TransactionType};
use reporting::Reporter;
use serde_json::{self, Value};
use std::borrow::Cow;
use std::fmt;

pub struct Cashflow;

#[derive(Default, Serialize)]
pub struct CashflowReport {
    pub credit: Money,
    pub debit:  Money,
}

impl Reporter for Cashflow {
    fn report<'a, I>(&self, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>>,
    {
        let cashflow: CashflowReport = transactions.fold(Default::default(), |mut acc, ref t| {
            match t.transaction_type {
                TransactionType::Credit => acc.credit += t.amount,
                TransactionType::Debit => acc.debit += t.amount,
                _ => {},
            }
            acc
        });

        serde_json::to_value(&cashflow).expect("could not calculate cashflow report")
    }

    fn key(&self) -> Option<String> {
        Some("cashflow".to_owned())
    }
}

impl fmt::Display for CashflowReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "In: ${:0.2}  Out: ${:0.2}  Delta: ${:0.2}",
            self.credit,
            self.debit,
            self.credit - self.debit
        )
    }
}

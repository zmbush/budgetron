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
use std::collections::HashMap;

pub struct Categories;

#[derive(Default, Serialize)]
pub struct CategoryEntry {
    amount:       Money,
    transactions: Vec<String>,
}

impl Reporter for Categories {
    fn report<'a, I>(&self, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>>,
    {
        let mut categories = HashMap::new();
        for transaction in transactions {
            let entry: &mut CategoryEntry = categories
                .entry(transaction.category.clone())
                .or_insert_with(Default::default);
            entry.amount += match transaction.transaction_type {
                TransactionType::Credit => transaction.amount,
                TransactionType::Debit => -transaction.amount,
                _ => Money::zero(),
            };
            entry.transactions.push(transaction.uid());
        }

        serde_json::to_value(categories).expect("Unable to serialize categories")
    }

    fn key(&self) -> Option<String> {
        Some("categories".to_string())
    }
}

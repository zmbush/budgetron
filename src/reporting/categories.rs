// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    crate::{
        loading::{Money, Transaction, TransactionType},
        reporting::{config::ReportOptions, timeseries::Timeseries, Reporter},
    },
    budgetronlib::fintime::Date,
    serde::Serialize,
    serde_json::{self, Value},
    std::{borrow::Cow, collections::HashMap},
};

pub struct Categories {
    options: ReportOptions,
}

impl Categories {
    pub fn with_options(options: ReportOptions) -> Categories {
        Categories { options }
    }
}

#[derive(Default, Serialize)]
pub struct CategoryEntry {
    amount: Money,
    transactions: Vec<String>,
}

#[derive(Default, Serialize)]
pub struct CategoriesReport {
    categories: HashMap<String, CategoryEntry>,
    timeseries: Option<Timeseries<HashMap<String, Money>>>,
}

impl CategoriesReport {
    fn ts_data(&self) -> HashMap<String, Money> {
        self.categories
            .iter()
            .map(|(name, entry)| (name.to_owned(), entry.amount))
            .collect()
    }
}

impl Reporter for Categories {
    fn report<'a, I>(&self, transactions: I, _: Date) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>>,
    {
        let mut report = CategoriesReport {
            timeseries: if self.options.include_graph {
                Some(Timeseries::new())
            } else {
                None
            },
            ..Default::default()
        };
        for transaction in transactions {
            {
                let entry: &mut CategoryEntry = report
                    .categories
                    .entry(transaction.category.clone())
                    .or_insert_with(Default::default);
                entry.amount += match transaction.transaction_type {
                    TransactionType::Credit => transaction.amount,
                    TransactionType::Debit => -transaction.amount,
                    _ => Money::zero(),
                };
                entry.transactions.push(transaction.uid());
            }
            let ts_data = report.ts_data();
            if let Some(ref mut ts) = report.timeseries {
                ts.add(transaction.date, ts_data);
            }
        }

        serde_json::to_value(report).expect("Unable to serialize categories")
    }

    fn key(&self) -> Option<String> {
        Some("categories".to_string())
    }
}

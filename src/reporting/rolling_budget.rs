// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use budgetronlib::fintime::Date;
use loading::{Money, Transaction, TransactionType};
use reporting::Reporter;
use serde_json::{self, Value};
use std::borrow::Cow;
use std::collections::HashMap;
use reporting::timeseries::Timeseries;
use reporting::config::ReportOptions;

#[derive(Debug, Deserialize)]
pub struct RollingBudgetConfig {
    rolling_budget: RollingBudget,
}

#[derive(Debug, Deserialize)]
pub struct RollingBudget {
    start_date: Date,
    split: String,
    amounts: HashMap<String, Money>,
    options: ReportOptions,
}

impl RollingBudget {
    pub fn new_param(start_date: Date,
                     split: String,
                     amounts: HashMap<String, Money>,
                     options: ReportOptions)
                     -> RollingBudget {
        RollingBudget {
            start_date,
            split,
            amounts,
            options,
        }
    }

    pub fn new(cfg: RollingBudgetConfig) -> RollingBudget {
        cfg.rolling_budget
    }
}

#[derive(Debug, Serialize)]
pub struct RollingBudgetReport {
    budgets: HashMap<String, Money>,
    transactions: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timeseries: Option<Timeseries<HashMap<String, Money>>>,
}

impl RollingBudget {
    fn should_split(&self, transaction: &Transaction) -> bool {
        transaction.person == self.split
    }

    fn should_include(&self, transaction: &Transaction) -> bool {
        transaction.date >= self.start_date &&
        TransactionType::Transfer != transaction.transaction_type
    }

    fn proportions(&self) -> HashMap<&str, f64> {
        let total = self.amounts.values().sum::<Money>().to_f64();
        self.amounts
            .iter()
            .map(|(k, v)| (k.as_ref(), v.to_f64() / total))
            .collect()
    }

    fn split_transaction(&self, transaction: &Transaction) -> HashMap<String, Money> {
        if self.should_split(transaction) {
            self.proportions()
                .into_iter()
                .map(|(k, v)| (k.to_string(), transaction.amount * v))
                .collect()
        } else {
            let mut s = HashMap::new();
            s.insert(transaction.person.clone(), transaction.amount);
            s
        }
    }
}

impl Reporter for RollingBudget {
    fn report<'a, I>(&self, transactions: I) -> Value
        where I: Iterator<Item = Cow<'a, Transaction>>
    {
        let mut report = RollingBudgetReport {
            budgets: self.amounts.clone(),
            transactions: Vec::new(),
            timeseries: if self.options.include_graph {
                Some(Timeseries::new())
            } else {
                None
            },
        };
        let mut month = self.start_date.month();

        if let Some(ref mut ts) = report.timeseries {
            ts.add(self.start_date, self.amounts.clone());
        }
        for transaction in transactions {
            if self.should_include(&transaction) {
                if transaction.date.month() == 8 && transaction.date.year() == 2018 &&
                   transaction.date.day() == 10 {
                    println!("LORG::: {:?}", transaction);
                }
                if transaction.date.month() != month {
                    month = transaction.date.month();
                    for (name, amount) in &self.amounts {
                        *report
                             .budgets
                             .entry(name.to_string())
                             .or_insert_with(Money::zero) += *amount;
                    }
                }
                for (name, amount) in self.split_transaction(&transaction) {
                    let entry = report
                        .budgets
                        .entry(name.to_string())
                        .or_insert_with(Money::zero);
                    match transaction.transaction_type {
                        TransactionType::Debit => *entry -= amount,
                        TransactionType::Credit => *entry += amount,
                        _ => {},
                    }
                }
                report.transactions.push(transaction.uid());
                if let Some(ref mut ts) = report.timeseries {
                    ts.add(transaction.date, report.budgets.clone());
                }
            }
        }
        serde_json::to_value(&report).expect("Couldn't serialize")
    }

    fn key(&self) -> Option<String> {
        Some("rolling_budget".to_owned())
    }
}

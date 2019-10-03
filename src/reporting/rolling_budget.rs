// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::loading::{Money, Transaction, TransactionType};
use crate::reporting::config::ReportOptions;
use crate::reporting::timeseries::Timeseries;
use crate::reporting::Reporter;
use budgetronlib::fintime::Date;
use serde_derive::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::borrow::Cow;
use std::collections::HashMap;

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
    pub fn new_param(
        start_date: Date,
        split: String,
        amounts: HashMap<String, Money>,
        options: ReportOptions,
    ) -> RollingBudget {
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

#[derive(Debug, Serialize, Default)]
pub struct ExpenseBreakdown {
    split_transactions: Money,
    personal_transactions: Money,
}

#[derive(Debug, Serialize)]
pub struct RollingBudgetReport {
    budgets: HashMap<String, Money>,
    breakdown: HashMap<String, ExpenseBreakdown>,
    transactions: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timeseries: Option<Timeseries<HashMap<String, Money>>>,
}

impl RollingBudget {
    fn should_split(&self, transaction: &Transaction) -> bool {
        transaction.person == self.split
    }

    fn should_include(&self, transaction: &Transaction) -> bool {
        transaction.date >= self.start_date
            && TransactionType::Transfer != transaction.transaction_type
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
    where
        I: Iterator<Item = Cow<'a, Transaction>>,
    {
        let mut report = RollingBudgetReport {
            budgets: self.amounts.clone(),
            breakdown: HashMap::new(),
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
                if transaction.date.month() != month {
                    let mut count = transaction.date.month() as i32 - month as i32;
                    if count < 0 {
                        count += 12;
                    }
                    println!("Count: '{}'", count);
                    month = transaction.date.month();
                    for (name, amount) in &self.amounts {
                        *report
                            .budgets
                            .entry(name.to_string())
                            .or_insert_with(Money::zero) += (*amount) * count;
                    }
                }
                let split = self.should_split(&transaction);
                for (name, amount) in self.split_transaction(&transaction) {
                    let entry = report
                        .budgets
                        .entry(name.to_string())
                        .or_insert_with(Money::zero);
                    let breakdown_entry = report
                        .breakdown
                        .entry(name.to_string())
                        .or_insert_with(Default::default);
                    match transaction.transaction_type {
                        TransactionType::Debit => {
                            *entry -= amount;
                            if split {
                                breakdown_entry.split_transactions -= amount;
                            } else {
                                breakdown_entry.personal_transactions -= amount;
                            }
                        }
                        TransactionType::Credit => {
                            *entry += amount;
                            if split {
                                breakdown_entry.split_transactions += amount;
                            } else {
                                breakdown_entry.personal_transactions += amount;
                            }
                        }
                        _ => {}
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

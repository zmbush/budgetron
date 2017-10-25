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

#[derive(Debug, Deserialize)]
pub struct RollingBudgetConfig {
    rolling_budget: RollingBudget,
}

#[derive(Debug, Deserialize)]
pub struct RollingBudget {
    start_date: Date,
    split:      String,
    amounts:    HashMap<String, Money>,
}

impl RollingBudget {
    pub fn new_param(
        start_date: Date,
        split: String,
        amounts: HashMap<String, Money>,
    ) -> RollingBudget {
        RollingBudget {
            start_date,
            split,
            amounts,
        }
    }

    pub fn new(cfg: RollingBudgetConfig) -> RollingBudget {
        cfg.rolling_budget
    }
}

#[derive(Debug, Serialize)]
pub struct RollingBudgetReport {
    budgets: HashMap<String, Money>,
}

impl RollingBudget {
    fn should_split(&self, transaction: &Transaction) -> bool {
        transaction.person == self.split
    }

    fn should_include(&self, transaction: &Transaction) -> bool {
        transaction.date >= self.start_date
            && TransactionType::Transfer != transaction.transaction_type
    }

    fn proportions(&self) -> HashMap<&str, Money> {
        let total: Money = self.amounts.values().sum();
        self.amounts
            .iter()
            .map(|(k, v)| (k.as_ref(), v / total))
            .collect()
    }

    fn split_transaction(&self, transaction: &Transaction) -> HashMap<String, Money> {
        let mut splits = HashMap::new();
        if self.should_split(transaction) {
            splits = self.proportions()
                .iter()
                .map(|(k, v)| (k.to_string(), transaction.amount * *v))
                .collect();
        } else {
            splits.insert(transaction.person.clone(), transaction.amount);
        }
        splits
    }
}

impl Reporter for RollingBudget {
    fn report<'a, I>(&self, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>>,
    {
        let mut report = RollingBudgetReport {
            budgets: self.amounts.clone(),
        };
        let mut month = self.start_date.month();

        for transaction in transactions {
            if self.should_include(&transaction) {
                if transaction.date.month() != month {
                    month = transaction.date.month();
                    for (name, amount) in &self.amounts {
                        *report
                            .budgets
                            .entry(name.to_string())
                            .or_insert(Money::zero()) += *amount;
                    }
                }
                for (name, amount) in self.split_transaction(&transaction) {
                    let entry = report
                        .budgets
                        .entry(name.to_string())
                        .or_insert(Money::zero());
                    match transaction.transaction_type {
                        TransactionType::Debit => *entry -= amount,
                        TransactionType::Credit => *entry += amount,
                        _ => {},
                    }
                }
            }
        }
        serde_json::to_value(&report).expect("Couldn't serialize")
    }

    fn key(&self) -> Option<String> {
        Some("rolling_budget".to_owned())
    }
}

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
    serde::{Deserialize, Serialize},
    serde_json::{self, Value},
    std::{borrow::Cow, collections::HashMap},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct RollingBudget {
    split: String,
    rollover_months: Option<u8>,
    amounts: HashMap<Date, HashMap<String, Money>>,
    #[serde(default)]
    options: ReportOptions,
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
        self.amounts.keys().any(|&k| transaction.date >= k)
            && TransactionType::Transfer != transaction.transaction_type
    }

    fn proportions(amounts: &HashMap<String, Money>) -> HashMap<&str, f64> {
        let total = amounts.values().sum::<Money>().to_f64();
        amounts
            .iter()
            .map(|(k, v)| (k.as_ref(), v.to_f64() / total))
            .collect()
    }

    fn split_transaction(
        &self,
        transaction: &Transaction,
        amounts: &HashMap<String, Money>,
    ) -> HashMap<String, Money> {
        if self.should_split(transaction) {
            RollingBudget::proportions(amounts)
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
        let start_dates = {
            let mut sd = self.amounts.keys().collect::<Vec<_>>();
            sd.sort();
            sd
        };

        let mut amount_index = 0;
        let mut amounts = &self.amounts[start_dates[amount_index]];

        let mut report = RollingBudgetReport {
            budgets: amounts.clone(),
            breakdown: HashMap::new(),
            transactions: Vec::new(),
            timeseries: if self.options.include_graph {
                Some(Timeseries::new())
            } else {
                None
            },
        };
        let mut month = start_dates[amount_index].month();

        if let Some(ref mut ts) = report.timeseries {
            ts.add(start_dates[amount_index].clone(), amounts.clone());
        }
        for transaction in transactions {
            if self.should_include(&transaction) {
                if start_dates.len() > amount_index + 1
                    && transaction.date >= *start_dates[amount_index + 1]
                {
                    amount_index += 1;
                    amounts = &self.amounts[start_dates[amount_index]];
                }
                if transaction.date.month() != month {
                    let mut count = transaction.date.month() as i32 - month as i32;
                    if count < 0 {
                        count += 12;
                    }
                    month = transaction.date.month();
                    for (name, amount) in amounts {
                        let entry = report
                            .budgets
                            .entry(name.to_string())
                            .or_insert_with(Money::zero);
                        *entry += (*amount) * count;
                        if let Some(rollover_months) = self.rollover_months {
                            let max_saved = *amount * rollover_months;
                            if *entry > max_saved {
                                *entry = max_saved;
                            }
                        }
                    }
                }
                let split = self.should_split(&transaction);
                for (name, amount) in self.split_transaction(&transaction, &amounts) {
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

// Copyright 2019 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    crate::{
        loading::{Money, Transaction, TransactionType},
        reporting::Reporter,
    },
    serde::Serialize,
    serde_json::{self, Value},
    std::{borrow::Cow, collections::HashMap},
};

pub struct IncomeExpenseRatio {
    income_tags: Vec<String>,
    expense_tags: Vec<String>,
}

#[derive(Serialize, Default)]
pub struct IncomeExpenseData<'a> {
    by_tag: HashMap<String, Money>,
    other: Money,

    #[serde(skip)]
    tags: &'a [String],
}

#[derive(Serialize)]
pub struct IncomeExpenseReport<'a> {
    credit: IncomeExpenseData<'a>,
    debit: IncomeExpenseData<'a>,
}

impl IncomeExpenseRatio {
    pub fn new(income_tags: &[String], expense_tags: &[String]) -> Self {
        Self {
            income_tags: income_tags.to_owned(),
            expense_tags: expense_tags.to_owned(),
        }
    }
}

impl<'a> IncomeExpenseReport<'a> {
    fn new(income_tags: &'a [String], expense_tags: &'a [String]) -> Self {
        Self {
            credit: IncomeExpenseData::new(income_tags),
            debit: IncomeExpenseData::new(expense_tags),
        }
    }
}

impl<'a> IncomeExpenseData<'a> {
    fn new(tags: &'a [String]) -> Self {
        Self {
            tags,
            ..Default::default()
        }
    }

    fn update(&mut self, transaction: &Transaction) {
        match self.find_tag(&transaction.tags) {
            None => self.other += transaction.amount,
            Some(tag) => {
                *self
                    .by_tag
                    .entry(tag.to_owned())
                    .or_insert_with(Money::zero) += transaction.amount
            }
        }
    }

    fn find_tag(&self, transaction_tags: &[String]) -> Option<&'a str> {
        for tag in self.tags {
            if transaction_tags.contains(&tag) {
                return Some(tag);
            }
        }
        None
    }
}

impl Reporter for IncomeExpenseRatio {
    fn report<'a, I>(&self, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>>,
    {
        let mut report = IncomeExpenseReport::new(&self.income_tags, &self.expense_tags);

        for transaction in transactions {
            match transaction.transaction_type {
                TransactionType::Credit => report.credit.update(&transaction),
                TransactionType::Debit => report.debit.update(&transaction),
                _ => {}
            }
        }

        serde_json::to_value(report).expect("Couldn't calculate income_expense_ratio")
    }

    fn key(&self) -> Option<String> {
        Some("income_expense_ratio".to_owned())
    }
}

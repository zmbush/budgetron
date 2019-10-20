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
        reporting::{data::ConcreteReport, Reporter},
    },
    serde::{Deserialize, Serialize},
    std::{borrow::Cow, collections::HashMap},
};

#[cfg(target_arch = "wasm32")]
use {crate::reporting::web::ConfiguredReportDataUi, yew::prelude::*};

pub struct IncomeExpenseRatio {
    income_tags: Vec<String>,
    expense_tags: Vec<String>,
}

#[derive(Default)]
struct IncomeExpenseData<'a> {
    by_tag: HashMap<String, Money>,
    other: Money,

    tags: &'a [String],
}

struct IncomeExpenseReport<'a> {
    credit: IncomeExpenseData<'a>,
    debit: IncomeExpenseData<'a>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct IncomeExpenseReportData {
    by_tag: HashMap<String, Money>,
    other: Money,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct IncomeExpenseReportType {
    credit: IncomeExpenseReportData,
    debit: IncomeExpenseReportData,
}

#[cfg(target_arch = "wasm32")]
impl IncomeExpenseReportType {
    pub fn view(
        &self,
        _transactions: &HashMap<String, Transaction>,
    ) -> Html<ConfiguredReportDataUi> {
        html! {}
    }
}

impl<'a> From<IncomeExpenseData<'a>> for IncomeExpenseReportData {
    fn from(rep: IncomeExpenseData<'a>) -> Self {
        Self {
            by_tag: rep.by_tag,
            other: rep.other,
        }
    }
}

impl<'a> From<IncomeExpenseReport<'a>> for IncomeExpenseReportType {
    fn from(rep: IncomeExpenseReport) -> Self {
        Self {
            credit: rep.credit.into(),
            debit: rep.debit.into(),
        }
    }
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
    fn report<'t>(
        &self,
        transactions: impl Iterator<Item = Cow<'t, Transaction>>,
    ) -> ConcreteReport {
        let mut report = IncomeExpenseReport::new(&self.income_tags, &self.expense_tags);

        for transaction in transactions {
            match transaction.transaction_type {
                TransactionType::Credit => report.credit.update(&transaction),
                TransactionType::Debit => report.debit.update(&transaction),
                _ => {}
            }
        }

        IncomeExpenseReportType::from(report).into()
    }
}

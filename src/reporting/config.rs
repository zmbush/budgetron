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
        reporting::{Cashflow, Categories, IncomeExpenseRatio, Reporter, RollingBudget},
    },
    budgetronlib::fintime::Date,
    serde_derive::{Deserialize, Serialize},
    serde_json::{self, Value},
    std::{borrow::Cow, collections::HashMap},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfiguredReports {
    report: Vec<Report>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Report {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_type: Option<TransactionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_owners: Option<Vec<String>>,
    config: ReportType,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    old_configs: Vec<HistoricalConfig>,
    #[serde(default)]
    ui_config: UIConfig,

    #[serde(skip_serializing_if = "is_false", default)]
    by_week: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    by_month: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    by_quarter: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    by_year: bool,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_false(value: &bool) -> bool {
    !value
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HistoricalConfig {
    end_date: Date,
    config: ReportType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ReportType {
    RollingBudget {
        start_date: Date,
        split: String,
        amounts: HashMap<String, Money>,
        #[serde(default)]
        options: ReportOptions,
    },
    Cashflow {
        #[serde(default)]
        options: ReportOptions,
    },
    Categories {
        #[serde(default)]
        options: ReportOptions,
    },
    IncomeExpenseRatio {
        #[serde(default)]
        income_tags: Vec<String>,
        #[serde(default)]
        expense_tags: Vec<String>,
        #[serde(default)]
        options: ReportOptions,
    },
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ReportOptions {
    pub include_graph: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UIConfig {
    #[serde(default = "default_true")]
    show_diff: bool,
    #[serde(default)]
    expenses_only: bool,
}

impl Default for UIConfig {
    fn default() -> Self {
        UIConfig {
            show_diff: true,
            expenses_only: false,
        }
    }
}

fn default_true() -> bool {
    true
}

impl Report {
    fn inner_run_report<'a, I, R>(&self, reporter: &R, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>> + Clone,
        R: Reporter,
    {
        let mut retval = serde_json::map::Map::new();

        macro_rules! check_by {
            ($($name:ident),*) => {
                $(if self.$name {
                    retval.insert(
                        stringify!($name).to_owned(),
                        reporter.$name().report(transactions.clone()),
                    );
                })*

                if !($(self.$name)||*) {
                    reporter.report(transactions)
                } else {
                    Value::Object(retval)
                }
            }
        }

        check_by! {
            by_week, by_month, by_quarter, by_year
        }
    }

    fn filter_report_only_owners<'a, I, R>(&self, reporter: &R, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>> + Clone,
        R: Reporter,
    {
        if let Some(ref only_owners) = self.only_owners {
            self.inner_run_report(&reporter.only_owners(only_owners.clone()), transactions)
        } else {
            self.inner_run_report(reporter, transactions)
        }
    }

    fn filter_report_only_type<'a, I, R>(&self, reporter: &R, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>> + Clone,
        R: Reporter,
    {
        if let Some(only_type) = self.only_type {
            self.filter_report_only_owners(&reporter.only_type(only_type), transactions)
        } else {
            self.filter_report_only_owners(reporter, transactions)
        }
    }

    fn filter_report_only_tags<'a, I, R>(&self, reporter: &R, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>> + Clone,
        R: Reporter,
    {
        if let Some(ref only_tags) = self.only_tags {
            self.filter_report_only_type(&reporter.only_tags(only_tags.clone()), transactions)
        } else {
            self.filter_report_only_type(reporter, transactions)
        }
    }

    fn filter_report_skip_tags<'a, I, R>(&self, reporter: &R, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>> + Clone,
        R: Reporter,
    {
        if let Some(ref skip_tags) = self.skip_tags {
            self.filter_report_only_tags(&reporter.excluding_tags(skip_tags.clone()), transactions)
        } else {
            self.filter_report_only_tags(reporter, transactions)
        }
    }

    fn run_report<'a, I, R>(&self, reporter: &R, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>> + Clone,
        R: Reporter,
    {
        self.filter_report_skip_tags(reporter, transactions)
    }
}

impl Reporter for ConfiguredReports {
    fn report<'a, I>(&self, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>> + Clone,
    {
        let mut retval = Vec::new();
        for report_config in &self.report {
            let report_key = report_config
                .name
                .to_lowercase()
                .split(' ')
                .collect::<Vec<_>>()
                .join("_");
            let value = match report_config.config {
                ReportType::RollingBudget {
                    start_date,
                    ref split,
                    ref amounts,
                    ref options,
                } => report_config.run_report(
                    &RollingBudget::new_param(
                        start_date,
                        split.clone(),
                        amounts.clone(),
                        (*options).clone(),
                    ),
                    transactions.clone(),
                ),
                ReportType::Cashflow { ref options } => report_config.run_report(
                    &Cashflow::with_options((*options).clone()),
                    transactions.clone(),
                ),
                ReportType::Categories { ref options } => report_config.run_report(
                    &Categories::with_options((*options).clone()),
                    transactions.clone(),
                ),
                ReportType::IncomeExpenseRatio {
                    ref income_tags,
                    ref expense_tags,
                    ..
                } => report_config.run_report(
                    &IncomeExpenseRatio::new(income_tags, expense_tags),
                    transactions.clone(),
                ),
            };

            let mut report_data = serde_json::map::Map::new();
            report_data.insert("data".to_string(), value);
            report_data.insert(
                "report".to_string(),
                serde_json::to_value(report_config).expect("Could not write config"),
            );
            report_data.insert("key".to_string(), Value::String(report_key));
            retval.push(Value::Object(report_data));
        }
        Value::Array(retval)
    }

    fn key(&self) -> Option<String> {
        Some("reports".to_owned())
    }
}

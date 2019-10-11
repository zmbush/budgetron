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
        reporting::{
            filters::IterExt, meta::by_timeframe::ByTimeframe, Cashflow, Categories,
            IncomeExpenseRatio, Reporter, RollingBudget,
        },
    },
    budgetronlib::fintime::{Date, Timeframe},
    serde::{Deserialize, Serialize},
    std::{
        borrow::Cow,
        collections::{BTreeMap, HashMap},
    },
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfiguredReports {
    report: Vec<ReportConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReportConfig {
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HistoricalConfig {
    end_date: Date,
    config: ReportType,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
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

impl ReportConfig {
    fn run_report<'r, 't, I, R>(
        &self,
        reporter: &'r R,
        transactions: I,
    ) -> Vec<ConfiguredReportDataInner>
    where
        I: Iterator<Item = Cow<'t, Transaction>>,
        R: Reporter,
    {
        let mut transactions: Box<dyn Iterator<Item = Cow<'t, Transaction>>> =
            Box::new(transactions);

        if let Some(ref skip_tags) = self.skip_tags {
            transactions = Box::new(transactions.excluding_tags(skip_tags.clone()));
        }

        if let Some(ref only_tags) = self.only_tags {
            transactions = Box::new(transactions.only_tags(only_tags.clone()));
        }

        if let Some(only_type) = self.only_type {
            transactions = Box::new(transactions.only_type(only_type));
        }

        if let Some(ref only_owners) = self.only_owners {
            transactions = Box::new(transactions.only_owners(only_owners.clone()));
        }

        let transactions = transactions.collect::<Vec<_>>().into_iter();

        macro_rules! check_by {
            ($($name:ident => $timeframe:expr),*) => {
                let mut retval = Vec::new();
                $(if self.$name {
                    retval.push(ConfiguredReportDataInner::ByTimeframe {
                        timeframe: $timeframe,
                        data: ByTimeframe::new(reporter, $timeframe).report(transactions.clone())
                    });
                })*

                if !($(self.$name)||*) {
                    vec![ConfiguredReportDataInner::Simple(reporter.report(transactions))]
                } else {
                    retval
                }
            }
        }

        check_by! {
            by_week => Timeframe::Weeks(1),
            by_month => Timeframe::Months(1),
            by_quarter => Timeframe::Quarters(1),
            by_year => Timeframe::Years(1)
        }
    }
}

trait SizedSerialize: Serialize + Sized {}
impl<T: Serialize + Sized> SizedSerialize for T {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "format")]
pub enum ConfiguredReportDataInner {
    ByTimeframe {
        timeframe: budgetronlib::fintime::Timeframe,
        data: BTreeMap<Date, super::data::ConcreteReport>,
    },

    Simple(super::data::ConcreteReport),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfiguredReportData {
    config: ReportConfig,
    data: ConfiguredReportDataInner,
}

impl ConfiguredReports {
    pub fn report<'t>(
        &self,
        transactions: impl Iterator<Item = Cow<'t, Transaction>> + Clone,
    ) -> Vec<ConfiguredReportData> {
        let mut retval = Vec::new();
        for report_config in &self.report {
            let results = match report_config.config {
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

            for data in results {
                retval.push(ConfiguredReportData {
                    data,
                    config: (*report_config).clone(),
                });
            }
        }

        retval
    }
}

use std::collections::HashMap;
use budgetronlib::fintime::Date;
use reporting::{Cashflow, Categories, Reporter, RollingBudget};
use std::borrow::Cow;
use serde_json::{self, Value};
use loading::{Transaction, TransactionType};

#[derive(Debug, Deserialize)]
pub struct ConfiguredReports {
    report: Vec<Report>,
}

#[derive(Debug, Deserialize)]
pub struct Report {
    name:      String,
    only_type: Option<TransactionType>,
    skip_tags: Option<Vec<String>>,
    config:    ReportType,

    #[serde(default)] by_week:    bool,
    #[serde(default)] by_month:   bool,
    #[serde(default)] by_quarter: bool,
    #[serde(default)] by_year:    bool,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ReportType {
    RollingBudget {
        start_date: Date,
        split:      String,
        amounts:    HashMap<String, f64>,
    },
    Cashflow,
    Categories,
}


impl Report {
    fn inner_run_report<'a, I, R>(&self, reporter: R, transactions: I) -> Value
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

    fn run_report<'a, I, R>(&self, reporter: R, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>> + Clone,
        R: Reporter,
    {
        match (&self.skip_tags, self.only_type) {
            (&Some(ref tags), Some(t)) => self.inner_run_report(
                reporter.only_type(t).excluding_tags(tags.clone()),
                transactions,
            ),
            (&Some(ref tags), None) => {
                self.inner_run_report(reporter.excluding_tags(tags.clone()), transactions)
            },
            (&None, Some(t)) => self.inner_run_report(reporter.only_type(t), transactions),
            (&None, None) => self.inner_run_report(reporter, transactions),
        }
    }
}

impl Reporter for ConfiguredReports {
    fn report<'a, I>(&self, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>> + Clone,
    {
        let mut retval = serde_json::map::Map::new();
        for report_config in &self.report {
            let report_key = report_config
                .name
                .to_lowercase()
                .split(" ")
                .collect::<Vec<_>>()
                .join("_");
            let value = match &report_config.config {
                &ReportType::RollingBudget {
                    start_date,
                    ref split,
                    ref amounts,
                } => report_config.run_report(
                    RollingBudget::new_param(start_date, split.clone(), amounts.clone()),
                    transactions.clone(),
                ),
                &ReportType::Cashflow => report_config.run_report(Cashflow, transactions.clone()),
                &ReportType::Categories => {
                    report_config.run_report(Categories, transactions.clone())
                },
            };
            retval.insert(report_key, value);
        }
        Value::Object(retval)
    }

    fn key(&self) -> Option<String> {
        None
    }
}

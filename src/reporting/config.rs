use std::collections::HashMap;
use budgetronlib::fintime::Date;
use reporting::{Cashflow, Categories, Reporter, RollingBudget};
use std::borrow::Cow;
use serde_json::{self, Value};
use loading::{Money, Transaction, TransactionType};

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfiguredReports {
    report: Vec<Report>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Report {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")] only_type: Option<TransactionType>,
    #[serde(skip_serializing_if = "Option::is_none")] skip_tags: Option<Vec<String>>,
    config: ReportType,

    #[serde(skip_serializing_if = "is_false", default)] by_week:    bool,
    #[serde(skip_serializing_if = "is_false", default)] by_month:   bool,
    #[serde(skip_serializing_if = "is_false", default)] by_quarter: bool,
    #[serde(skip_serializing_if = "is_false", default)] by_year:    bool,
}

fn is_false(value: &bool) -> bool {
    !value
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ReportType {
    RollingBudget {
        start_date: Date,
        split:      String,
        amounts:    HashMap<String, Money>,
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
            let mut report_data = serde_json::map::Map::new();
            report_data.insert("data".to_string(), value);
            report_data.insert(
                "config".to_string(),
                serde_json::to_value(report_config).expect("Could not write config"),
            );
            retval.insert(report_key, Value::Object(report_data));
        }
        Value::Object(retval)
    }

    fn key(&self) -> Option<String> {
        None
    }
}

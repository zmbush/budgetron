use std::collections::HashMap;
use budgetronlib::fintime::Date;
use reporting::{Cashflow, Reporter, RollingBudget};
use std::borrow::Cow;
use serde_json::{self, Value};
use loading::Transaction;

#[derive(Debug, Deserialize)]
pub struct ConfiguredReports {
    reports: Vec<Report>,
}

#[derive(Debug, Deserialize)]
pub struct Report {
    name: String,
    #[serde(default)] by_week: bool,
    #[serde(default)] by_month: bool,
    #[serde(default)] by_quarter: bool,
    #[serde(default)] by_year: bool,

    skip_tags: Option<Vec<String>>,
    config: ReportType,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ReportType {
    RollingBudget {
        start_date: Date,
        split: String,
        amounts: HashMap<String, f64>,
    },
    Cashflow,
    //Categories,
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
        if let Some(ref tags) = self.skip_tags {
            self.inner_run_report(reporter.excluding_tags(tags.clone()), transactions)
        } else {
            self.inner_run_report(reporter, transactions)
        }
    }
}

impl Reporter for ConfiguredReports {
    fn report<'a, I>(&self, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>> + Clone,
    {
        let mut retval = serde_json::map::Map::new();
        for report_config in &self.reports {
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
            };
            let value = match value {
                Value::Object(mut o) => {
                    o.insert("name".to_owned(), Value::String(report_config.name.clone()));
                    Value::Object(o)
                },
                other => other,
            };
            retval.insert(report_key, value);
        }
        Value::Object(retval)
    }

    fn key(&self) -> Option<String> {
        None
    }
}

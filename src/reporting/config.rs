use std::collections::HashMap;
use budgetronlib::fintime::Date;
use reporting::{Cashflow, Reporter, RollingBudget};
use std::borrow::Cow;
use serde_json::{self, Value};
use loading::Transaction;

#[derive(Debug, Deserialize)]
pub struct ConfiguredReports {
    reports: HashMap<String, Report>,
}

#[derive(Debug, Deserialize)]
pub struct Report {
    #[serde(default)] by_month: bool,
    #[serde(default)] by_quarter: bool,
    #[serde(default)] by_year: bool,

    skip_tags: Option<Vec<String>>,
    report: ReportType,
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
        if self.by_month {
            retval.insert(
                "by_month".to_owned(),
                reporter.by_month().report(transactions.clone()),
            );
        }
        if self.by_quarter {
            retval.insert(
                "by_quarter".to_owned(),
                reporter.by_quarter().report(transactions.clone()),
            );
        }
        if self.by_year {
            retval.insert(
                "by_year".to_owned(),
                reporter.by_year().report(transactions.clone()),
            );
        }

        if !(self.by_month || self.by_quarter | self.by_year) {
            reporter.report(transactions)
        } else {
            Value::Object(retval)
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
        for (name, report_config) in &self.reports {
            retval.insert(
                name.to_owned(),
                match &report_config.report {
                    &ReportType::RollingBudget {
                        start_date,
                        ref split,
                        ref amounts,
                    } => report_config.run_report(
                        RollingBudget::new_param(start_date, split.clone(), amounts.clone()),
                        transactions.clone(),
                    ),
                    &ReportType::Cashflow => {
                        report_config.run_report(Cashflow, transactions.clone())
                    },
                },
            );
        }
        Value::Object(retval)
    }

    fn key(&self) -> Option<String> {
        None
    }
}

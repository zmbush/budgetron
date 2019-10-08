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
    serde::Serialize,
    serde_json::{self, Value},
    std::{borrow::Cow, fmt},
};

pub struct Cashflow {
    options: ReportOptions,
}

#[derive(Default, Serialize)]
pub struct CashflowReport {
    credit: Money,
    debit: Money,
    net: Money,
    timeseries: Option<Timeseries<CashflowDatum>>,
}

#[derive(Default, Serialize)]
pub struct CashflowDatum {
    credit: Money,
    debit: Money,
    net: Money,
}

impl Cashflow {
    pub fn with_options(options: ReportOptions) -> Cashflow {
        Cashflow { options }
    }
}

impl CashflowReport {
    fn datum(&self) -> CashflowDatum {
        CashflowDatum {
            credit: self.credit,
            debit: self.debit,
            net: self.net,
        }
    }
}

impl Reporter for Cashflow {
    fn report<'a, I>(&self, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>>,
    {
        let report = CashflowReport {
            timeseries: if self.options.include_graph {
                Some(Timeseries::new())
            } else {
                None
            },
            ..Default::default()
        };

        let cashflow: CashflowReport = transactions.fold(report, |mut report, ref t| {
            match t.transaction_type {
                TransactionType::Credit => {
                    report.credit += t.amount;
                    report.net += t.amount;
                }
                TransactionType::Debit => {
                    report.debit += t.amount;
                    report.net -= t.amount;
                }
                _ => {}
            }
            let datum = report.datum();
            if let Some(ref mut ts) = report.timeseries {
                ts.add(t.date, datum);
            }
            report
        });

        serde_json::to_value(&cashflow).expect("could not calculate cashflow report")
    }

    fn key(&self) -> Option<String> {
        Some("cashflow".to_owned())
    }
}

impl fmt::Display for CashflowReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "In: ${:0.2}  Out: ${:0.2}  Delta: ${:0.2}",
            self.credit,
            self.debit,
            self.credit - self.debit
        )
    }
}

use loading::{Transaction, TransactionType};
use reporting::Reporter;
use serde_json::{self, Value};
use serde_json::map::Map;
use std::borrow::Cow;
use std::fmt;

pub struct Cashflow;

#[derive(Default, Serialize)]
pub struct CashflowReport {
    pub credit: f64,
    pub debit: f64,
}

impl Reporter for Cashflow {
    fn report<'a, I>(&self, transactions: I) -> Value
        where I: Iterator<Item = Cow<'a, Transaction>>
    {
        let cashflow: CashflowReport = transactions.fold(Default::default(), |mut acc, ref t| {
            match t.transaction_type {
                TransactionType::Credit => acc.credit += t.amount,
                TransactionType::Debit => acc.debit += t.amount,
                _ => {},
            }
            acc
        });

        serde_json::to_value(&cashflow).expect("could not calculate cashflow report")
    }

    fn key(&self) -> Option<String> {
        Some("cashflow".to_owned())
    }

    fn description(&self) -> Vec<String> {
        vec!["Cashflow".to_owned()]
    }
}

impl CashflowReport {
    pub fn print(&self) {
        println!("{}", self)
    }
}

impl fmt::Display for CashflowReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,
                 "In: ${:0.2}  Out: ${:0.2}  Delta: ${:0.2}",
                 self.credit,
                 self.debit,
                 self.credit - self.debit)
    }
}

use loading::{Transaction, TransactionType};
use reporting::Reporter;
use std::fmt;

pub struct Cashflow;

#[derive(Default, Serialize)]
pub struct CashflowReport {
    pub credit: f64,
    pub debit: f64,
}

impl Reporter for Cashflow {
    type OutputType = CashflowReport;

    fn report<'a, I>(&self, transactions: I) -> CashflowReport
        where I: Iterator<Item = &'a Transaction>
    {
        transactions.fold(Default::default(), |mut acc, &ref t| {
            match t.transaction_type {
                TransactionType::Credit => acc.credit += t.amount,
                TransactionType::Debit => acc.debit += t.amount,
                _ => {},
            }
            acc
        })
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

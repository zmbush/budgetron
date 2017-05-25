use loading::Transaction;
use std::fmt;

pub trait Reporter {
    type OutputType;

    fn report(&self, transactions: &Vec<Transaction>) -> Self::OutputType;
}

mod net_worth;
mod database;

pub use reporting::database::Database;
pub use reporting::net_worth::NetWorth;

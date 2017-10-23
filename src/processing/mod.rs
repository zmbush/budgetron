use budgetronlib::error::BResult;
use loading::Transaction;

pub mod config;
mod regex;
mod transfers;

pub enum Collator {
    Transfers(transfers::TransferCollator),
    Config(config::ConfiguredProcessors),
}

pub use processing::transfers::TransferCollator;
pub use processing::config::ConfiguredProcessors;

pub trait Collate {
    fn collate(&self, transactions: Vec<Transaction>) -> BResult<Vec<Transaction>>;
}

impl Collate for Collator {
    fn collate(&self, transactions: Vec<Transaction>) -> BResult<Vec<Transaction>> {
        match *self {
            Collator::Transfers(ref tc) => tc.collate(transactions),
            Collator::Config(ref cfg) => cfg.collate(transactions),
        }
    }
}

pub fn collate_all(
    mut transactions: Vec<Transaction>,
    collators: Vec<Collator>,
) -> BResult<Vec<Transaction>> {
    for ref collator in collators {
        transactions = collator.collate(transactions)?
    }
    Ok(transactions)
}

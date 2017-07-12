use budgetronlib::error::BResult;
use loading::Transaction;

mod transfers;
mod tags;

pub enum Collator {
    Transfers(transfers::TransferCollator),
    Tags(tags::TagCollator),
}

pub use processing::tags::{TagCollator, TagCollatorConfig};
pub use processing::transfers::TransferCollator;

pub trait Collate {
    fn collate(&self, transactions: Vec<Transaction>) -> BResult<Vec<Transaction>>;
}

impl Collate for Collator {
    fn collate(&self, transactions: Vec<Transaction>) -> BResult<Vec<Transaction>> {
        match *self {
            Collator::Transfers(ref tc) => tc.collate(transactions),
            Collator::Tags(ref tc) => tc.collate(transactions),
        }
    }
}

pub fn collate_all(mut transactions: Vec<Transaction>,
                   collators: Vec<Collator>)
                   -> BResult<Vec<Transaction>> {
    for ref collator in collators {
        transactions = collator.collate(transactions)?
    }
    Ok(transactions)
}

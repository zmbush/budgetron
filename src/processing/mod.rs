use budgetronlib::error::BResult;
use loading::Transaction;

mod regex;
mod transfers;
mod tags;
mod owner;

pub enum Collator {
    Transfers(transfers::TransferCollator),
    Tags(tags::TagCollator),
    Owners(owner::OwnersCollator),
}

pub use processing::owner::{OwnersConfig, OwnersCollator};
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
            Collator::Owners(ref oc) => oc.collate(transactions),
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

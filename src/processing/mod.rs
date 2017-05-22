use budgetronlib::error::BResult;
use loading::Transaction;

mod transfers;

pub use processing::transfers::TransferCollator;

pub trait Collator {
    fn collate(&self, transactions: Vec<Transaction>) -> BResult<Vec<Transaction>>;
}

pub fn collate_all(mut transactions: Vec<Transaction>,
                   collators: Vec<Box<Collator>>)
                   -> BResult<Vec<Transaction>> {
    for ref collator in collators {
        transactions = collator.collate(transactions)?
    }
    Ok(transactions)
}

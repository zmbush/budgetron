extern crate budgetronlib;
extern crate transactor;

use budgetronlib::error::BResult;
use transactor::Transaction;

trait Collator {
    fn collate(transactions: Vec<Transaction>) -> BResult<Vec<Transaction>>;
}

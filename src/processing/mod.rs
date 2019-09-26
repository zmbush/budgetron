// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use budgetronlib::error::BResult;
use loading::Transaction;

pub mod config;
mod refunds;
mod regex;
mod transfers;

pub enum Collator {
    Transfers(transfers::TransferCollator),
    Refund(refunds::RefundCollator),
    Config(config::ConfiguredProcessors),
}

pub use processing::config::ConfiguredProcessors;
pub use processing::refunds::RefundCollator;
pub use processing::transfers::TransferCollator;

pub trait Collate {
    fn collate(&self, transactions: Vec<Transaction>) -> BResult<Vec<Transaction>>;
}

impl Collate for Collator {
    fn collate(&self, transactions: Vec<Transaction>) -> BResult<Vec<Transaction>> {
        match *self {
            Collator::Transfers(ref tc) => tc.collate(transactions),
            Collator::Config(ref cfg) => cfg.collate(transactions),
            Collator::Refund(ref rc) => rc.collate(transactions),
        }
    }
}

pub fn collate_all(
    mut transactions: Vec<Transaction>,
    collators: &[Collator],
) -> BResult<Vec<Transaction>> {
    for collator in collators {
        transactions = collator.collate(transactions)?
    }
    Ok(transactions)
}

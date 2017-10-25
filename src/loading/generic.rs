// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use budgetronlib::error::BResult;
use loading::money::Money;
use budgetronlib::fintime::Date;

#[derive(Debug, Serialize, Copy, Deserialize, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum TransactionType {
    Credit,
    Debit,
    Transfer,
}

impl Default for TransactionType {
    fn default() -> TransactionType {
        TransactionType::Credit
    }
}

impl TransactionType {
    pub fn is_credit(&self) -> bool {
        TransactionType::Credit == *self
    }

    pub fn is_debit(&self) -> bool {
        TransactionType::Debit == *self
    }

    pub fn is_transfer(&self) -> bool {
        TransactionType::Transfer == *self
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Transaction {
    pub date: Date,
    pub description: String,
    pub amount: Money,
    pub transaction_type: TransactionType,
    pub person: String,
    pub original_description: String,
    pub category: String,
    pub original_category: String,
    pub account_name: String,
    pub labels: String,
    pub notes: String,
    pub transfer_destination_account: Option<String>,
    pub tags: Vec<String>,
}

pub trait Genericize {
    fn genericize(self) -> BResult<Transaction>;
}

impl Genericize for Transaction {
    fn genericize(self) -> BResult<Transaction> {
        Ok(self)
    }
}

impl Transaction {
    pub fn uid(&self) -> String {
        format!(
            "{}{}{}",
            self.date.uid(),
            self.amount.uid(),
            match self.transaction_type {
                TransactionType::Credit => "C",
                TransactionType::Debit => "D",
                TransactionType::Transfer => "T",
            }
        )
    }
}

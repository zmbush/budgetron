// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::loading::generic::{Genericize, Transaction, TransactionType};
use crate::loading::money::Money;
use budgetronlib::error::BResult;
use budgetronlib::fintime::Date;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LogixExport {
    account: String,
    date: Date,
    amount: Money,
    balance: Money,
    category: String,
    description: String,
    memo: String,
    notes: String,
}

impl Genericize for LogixExport {
    fn genericize(self) -> BResult<Transaction> {
        Ok(Transaction {
            uid: None,
            date: self.date,
            person: "".to_owned(),
            description: self.description.clone(),
            original_description: self.description,
            amount: self.amount.abs(),
            transaction_type: if self.amount.is_negative() {
                TransactionType::Debit
            } else {
                TransactionType::Credit
            },
            category: self.category.clone(),
            original_category: self.category,
            account_name: self.account,
            labels: self.memo,
            notes: self.notes,
            transfer_destination_account: None,
            tags: vec![],
        })
    }
}

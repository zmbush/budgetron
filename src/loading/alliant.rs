// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    crate::loading::{
        generic::{Genericize, Transaction, TransactionType},
        money::Money,
    },
    budgetronlib::{error::BResult, fintime::Date},
    serde::Deserialize,
};

// "id","account_id","reference_id","transaction_type","amount","posted_at",
// "created_at","nickname","original_name","merchant_id","updated_at",
// "check_number","account_name","tags"
#[derive(Debug, Deserialize)]
pub enum AlliantTransactionType {
    Debit,
    Credit,
}

impl Into<TransactionType> for AlliantTransactionType {
    fn into(self) -> TransactionType {
        match self {
            AlliantTransactionType::Debit => TransactionType::Debit,
            AlliantTransactionType::Credit => TransactionType::Credit,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AlliantExport {
    id: String,
    account_id: i64,
    reference_id: i64,
    transaction_type: AlliantTransactionType,
    amount: Money,
    posted_at: Date,
    created_at: Date,
    nickname: String,
    original_name: String,
    merchant_id: String,
    updated_at: Date,
    check_number: Option<i32>,
    account_name: String,
    tags: String,
}

impl Genericize for AlliantExport {
    fn genericize(self) -> BResult<Transaction> {
        Ok(Transaction {
            uid: Some(self.id),
            date: self.posted_at,
            person: "".to_owned(),
            description: self.nickname,
            original_description: self.original_name,
            amount: self.amount,
            transaction_type: self.transaction_type.into(),
            category: self.tags.clone(),
            original_category: self.tags,
            account_name: self.account_name,
            labels: "".to_owned(),
            notes: "".to_owned(),
            transfer_destination_account: None,
            tags: vec![],
        })
    }
}

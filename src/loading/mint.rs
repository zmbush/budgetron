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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MintTransactionType {
    Debit,
    Credit,
}

impl Into<TransactionType> for MintTransactionType {
    fn into(self) -> TransactionType {
        match self {
            MintTransactionType::Debit => TransactionType::Debit,
            MintTransactionType::Credit => TransactionType::Credit,
        }
    }
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MintExport {
    date: Date,
    description: String,
    #[serde(rename = "Original Description")]
    original_description: String,
    amount: Money,
    #[serde(rename = "Transaction Type")]
    transaction_type: MintTransactionType,
    category: String,
    #[serde(rename = "Account Name")]
    account_name: String,
    labels: String,
    notes: String,
}

impl Genericize for MintExport {
    fn genericize(self) -> BResult<Transaction> {
        Ok(Transaction {
            uid: None,
            date: self.date,
            person: "".to_owned(),
            description: self.description,
            original_description: self.original_description,
            amount: self.amount,
            transaction_type: self.transaction_type.into(),
            category: self.category.clone(),
            original_category: self.category,
            account_name: self.account_name,
            labels: self.labels,
            notes: self.notes,
            transfer_destination_account: None,
            tags: vec![],
        })
    }
}

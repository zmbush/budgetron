// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use data_store;
use loading::Transaction;
use loading::TransactionType;
use reporting::Reporter;
use serde_json::Value;
use std::borrow::Cow;

pub struct Database;
impl Reporter for Database {
    fn report<'a, I>(&self, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>>,
    {
        let db = data_store::Transactions::new_from_env();
        let mut all_transactions = Vec::new();
        for t in transactions.into_iter() {
            let t = t.into_owned();
            all_transactions.push(data_store::models::NewTransaction {
                date: t.date.date.naive_utc(),
                person: t.person,
                description: t.description,
                original_description: t.original_description,
                amount: t.amount.to_f64(),
                transaction_type: match t.transaction_type {
                    TransactionType::Debit => "Debit",
                    TransactionType::Credit => "Credit",
                    TransactionType::Transfer => "Transfer",
                },
                category: t.category,
                original_category: t.original_category,
                account_name: t.account_name,
                labels: t.labels,
                notes: t.notes,
                transfer_destination_account: t.transfer_destination_account,
                tags: t.tags,
            })
        }
        if !all_transactions.is_empty() {
            db.set_transactions(all_transactions);
        }

        return Value::Null;
    }

    fn key(&self) -> Option<String> {
        None
    }
}

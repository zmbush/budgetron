// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use loading::{Money, Transaction, TransactionType};
use reporting::Reporter;
use serde_json::{self, Value};
use std::borrow::Cow;
use std::collections::BTreeMap;

pub struct NetWorth;

impl Reporter for NetWorth {
    fn report<'a, I>(&self, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>>,
    {
        let mut worth = BTreeMap::new();
        for transaction in transactions {
            *worth
                .entry(transaction.account_name.clone())
                .or_insert_with(Money::zero) += match transaction.transaction_type {
                TransactionType::Credit => transaction.amount,
                TransactionType::Debit | TransactionType::Transfer => -transaction.amount,
            };
            if let TransactionType::Transfer = transaction.transaction_type {
                *worth
                    .entry(
                        transaction
                            .transfer_destination_account
                            .clone()
                            .expect("transfer records should have a transfer_destination_account"),
                    )
                    .or_insert_with(Money::zero) += transaction.amount;
            }
        }

        serde_json::to_value(worth).expect("Could not convert networth")
    }

    fn key(&self) -> Option<String> {
        Some("net_worth".to_owned())
    }
}

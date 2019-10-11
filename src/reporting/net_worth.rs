// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    crate::{
        loading::{Money, Transaction, TransactionType},
        reporting::{data::ConcreteReport, Reporter},
    },
    serde::{Deserialize, Serialize},
    std::{borrow::Cow, collections::BTreeMap},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct NetWorthReport(BTreeMap<String, Money>);

pub struct NetWorth;

impl Reporter for NetWorth {
    fn report<'t>(
        &self,
        transactions: impl Iterator<Item = Cow<'t, Transaction>>,
    ) -> ConcreteReport {
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

        NetWorthReport(worth).into()
    }
}

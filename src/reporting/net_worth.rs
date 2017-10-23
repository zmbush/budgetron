use loading::{Transaction, TransactionType};
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
            *worth.entry(transaction.account_name.clone()).or_insert(0.0) +=
                match transaction.transaction_type {
                    TransactionType::Credit => transaction.amount,
                    TransactionType::Debit => -transaction.amount,
                    TransactionType::Transfer => -transaction.amount,
                };
            if let TransactionType::Transfer = transaction.transaction_type {
                *worth
                    .entry(
                        transaction
                            .transfer_destination_account
                            .clone()
                            .expect("transfer records should have a transfer_destination_account"),
                    )
                    .or_insert(0.0) += transaction.amount;
            }
        }

        serde_json::to_value(worth).expect("Could not convert networth")
    }

    fn key(&self) -> Option<String> {
        Some("net_worth".to_owned())
    }
}

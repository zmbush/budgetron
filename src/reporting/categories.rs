use loading::{Money, Transaction, TransactionType};
use reporting::Reporter;
use serde_json::{self, Value};
use std::borrow::Cow;
use std::collections::HashMap;

pub struct Categories;

impl Reporter for Categories {
    fn report<'a, I>(&self, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>>,
    {
        let mut categories = HashMap::new();
        for transaction in transactions {
            *categories
                .entry(transaction.category.clone())
                .or_insert(Money::zero()) += match transaction.transaction_type {
                TransactionType::Credit => transaction.amount,
                TransactionType::Debit => -transaction.amount,
                _ => Money::zero(),
            }
        }

        serde_json::to_value(categories).expect("Unable to serialize categories")
    }

    fn key(&self) -> Option<String> {
        Some("categories".to_string())
    }
}

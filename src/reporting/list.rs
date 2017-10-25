use reporting::Reporter;
use serde_json::{self, Value};
use loading::Transaction;

use std::borrow::Cow;
use std::collections::HashMap;

pub struct List;
impl Reporter for List {
    fn report<'a, I>(&self, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>>,
    {
        let transaction_map = transactions
            .map(|t| (t.uid(), t))
            .collect::<HashMap<_, _>>();
        serde_json::to_value(&transaction_map).expect("Couldn't serialize")
    }

    fn key(&self) -> Option<String> {
        Some("transactions".to_owned())
    }
}

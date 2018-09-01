use reporting::Reporter;
use serde_json::{self, Value};
use loading::Transaction;
use budgetronlib::fintime::Timeframe;

use std::borrow::Cow;
use std::collections::HashMap;

pub struct List;
impl Reporter for List {
    fn report<'a, I>(&self, transactions: I) -> Value
        where I: Iterator<Item = Cow<'a, Transaction>>
    {
        let transactions = transactions.collect::<Vec<_>>();
        let start_date = transactions.last().map(|t| t.date).unwrap_or_default() -
                         Timeframe::Years(2);
        let transaction_map = transactions
            .into_iter()
            .filter_map(|t| if t.date >= start_date {
                            Some((t.uid(), t))
                        } else {
                            None
                        })
            .collect::<HashMap<_, _>>();
        serde_json::to_value(&transaction_map).expect("Couldn't serialize")
    }

    fn key(&self) -> Option<String> {
        Some("transactions".to_owned())
    }
}

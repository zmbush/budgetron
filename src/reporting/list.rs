use {
    crate::{loading::Transaction, reporting::Reporter},
    budgetronlib::fintime::{Date, Timeframe},
    serde_json::{self, Value},
    std::{borrow::Cow, collections::HashMap},
};

pub struct List;
impl Reporter for List {
    fn report<'a, I>(&self, transactions: I, _: Date) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>>,
    {
        let transactions = transactions.collect::<Vec<_>>();
        let start_date =
            transactions.last().map(|t| t.date).unwrap_or_default() - Timeframe::Years(3);
        let transaction_map = transactions
            .into_iter()
            .filter_map(|t| {
                if t.date >= start_date {
                    Some((t.uid(), t))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();
        serde_json::to_value(&transaction_map).expect("Couldn't serialize")
    }

    fn key(&self) -> Option<String> {
        Some("transactions".to_owned())
    }
}

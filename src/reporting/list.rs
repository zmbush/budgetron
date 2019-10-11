use {
    crate::loading::Transaction,
    budgetronlib::fintime::Timeframe,
    std::{borrow::Cow, collections::HashMap},
};

pub struct List;

impl List {
    pub fn report<'t>(
        &self,
        transactions: impl Iterator<Item = Cow<'t, Transaction>>,
    ) -> HashMap<String, Transaction> {
        let transactions = transactions.collect::<Vec<_>>();
        let start_date =
            transactions.last().map(|t| t.date).unwrap_or_default() - Timeframe::Years(3);
        transactions
            .into_iter()
            .filter_map(|t| {
                if t.date >= start_date {
                    Some((t.uid(), t.into_owned()))
                } else {
                    None
                }
            })
            .collect()
    }
}

use reporting::Reporter;
use std::borrow::Cow;
use loading::{Transaction, TransactionType};
use serde_json::Value;

pub struct OnlyType<'a, T>
where
    T: 'a + Reporter,
{
    inner: &'a T,
    t:     TransactionType,
}

impl<'a, T> OnlyType<'a, T>
where
    T: 'a + Reporter,
{
    pub fn new(inner: &'a T, t: TransactionType) -> Self {
        OnlyType { inner, t }
    }
}

impl<'a, T> Reporter for OnlyType<'a, T>
where
    T: Reporter,
{
    fn report<'b, I>(&self, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'b, Transaction>>,
    {
        let (transactions, _): (Vec<_>, Vec<_>) = transactions
            .into_iter()
            .partition(|t| t.transaction_type == self.t);

        self.inner.report(transactions.into_iter())
    }

    fn key(&self) -> Option<String> {
        Some(format!("only_type_{:?}", self.t))
    }
}

use loading::Transaction;
use reporting::Reporter;
use serde_json::Value;
use std::borrow::Cow;

pub struct ExcludingTags<'a, T>
where
    T: 'a + Reporter,
{
    inner: &'a T,
    tags: Vec<String>,
}

impl<'a, T> ExcludingTags<'a, T>
where
    T: 'a + Reporter,
{
    pub fn new(inner: &'a T, tags: Vec<String>) -> Self {
        ExcludingTags { inner, tags }
    }
}

impl<'a, T> Reporter for ExcludingTags<'a, T>
where
    T: Reporter,
{
    fn report<'b, I>(&self, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'b, Transaction>>,
    {
        let (_, transactions): (Vec<_>, Vec<_>) = transactions
            .into_iter()
            .partition(|t| self.tags.iter().any(|tag| t.tags.contains(tag)));

        self.inner.report(transactions.into_iter())
    }

    fn key(&self) -> Option<String> {
        Some(format!("excluding_tags_{}", self.tags.join("_")))
    }
}

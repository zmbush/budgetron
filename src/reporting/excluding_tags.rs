use loading::Transaction;
use reporting::Reporter;
use serde_json::{self, Value};
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

        let mut retval = serde_json::map::Map::new();
        retval.insert(
            "tags_excluded".to_owned(),
            Value::Array(self.tags.iter().map(|t| Value::String(t.clone())).collect()),
        );
        if let Some(v) = self.inner.key() {
            retval.insert(v.to_owned(), self.inner.report(transactions.into_iter()));
        } else {
            match self.inner.report(transactions.into_iter()) {
                Value::Object(o) => for (k, v) in o {
                    retval.insert(k, v);
                },
                other => {
                    retval.insert("excluding_tags".to_owned(), other);
                },
            }
        }
        Value::Object(retval)
    }

    fn key(&self) -> Option<String> {
        Some(format!("excluding_tags_{}", self.tags.join("_")))
    }
}

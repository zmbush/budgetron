use budgetronlib::error::BResult;
use loading::Transaction;
use processing::Collate;
use regex;
use serde::de::{self, Visitor, Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
struct Regex(regex::Regex);

#[derive(Debug, Deserialize)]
struct MoneyRange {
    low: f64,
    high: f64,
}

#[derive(Debug, Deserialize)]
struct Matchers {
    description: Option<Vec<Regex>>,
    range: Option<MoneyRange>,
}

struct CategoryRegexVisitor;
impl<'de> Visitor<'de> for CategoryRegexVisitor {
    type Value = Regex;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid regex string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Regex, E>
        where E: de::Error
    {
        match regex::Regex::new(value) {
            Ok(re) => Ok(Regex(re)),
            Err(e) => Err(E::custom(format!("Unable to parse `{}` as regex {}", value, e))),
        }
    }
}

impl<'de> Deserialize<'de> for Regex {
    fn deserialize<D>(deserializer: D) -> Result<Regex, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(CategoryRegexVisitor)
    }
}

#[derive(Debug, Deserialize)]
struct TagCategories {
    category: HashMap<String, Matchers>,
}

#[derive(Debug, Deserialize)]
pub struct TagCollatorConfig {
    tag: TagCategories,
}

pub struct TagCollator {
    config: TagCollatorConfig,
}

impl Collate for TagCollator {
    fn collate(&self, mut transactions: Vec<Transaction>) -> BResult<Vec<Transaction>> {
        for transaction in transactions.iter_mut() {
            for (key, value) in self.config.tag.category.iter() {
                if let Some(ref description) = value.description {
                    if description
                           .iter()
                           .any(|v| v.0.is_match(&transaction.original_description)) {
                        transaction.tags.push(key.clone());
                        continue;
                    }
                }

                if let Some(ref range) = value.range {
                    if transaction.amount >= range.low && transaction.amount < range.high {
                        transaction.tags.push(key.clone());
                    }
                }

            }
        }
        Ok(transactions)
    }
}

impl TagCollator {
    pub fn new(config: TagCollatorConfig) -> TagCollator {
        TagCollator { config }
    }
}

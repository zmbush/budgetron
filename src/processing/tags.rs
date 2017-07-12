use budgetronlib::error::BResult;
use loading::Transaction;
use processing::Collate;
use regex::Regex;
use serde::de::{self, Visitor, Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
struct CategoryRegex {
    re: Regex,
}

struct CategoryRegexVisitor;
impl<'de> Visitor<'de> for CategoryRegexVisitor {
    type Value = CategoryRegex;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid regex string")
    }

    fn visit_str<E>(self, value: &str) -> Result<CategoryRegex, E>
        where E: de::Error
    {
        match Regex::new(value) {
            Ok(re) => Ok(CategoryRegex { re }),
            Err(e) => Err(E::custom(format!("Unable to parse `{}` as regex {}", value, e))),
        }
    }
}

impl<'de> Deserialize<'de> for CategoryRegex {
    fn deserialize<D>(deserializer: D) -> Result<CategoryRegex, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(CategoryRegexVisitor)
    }
}

#[derive(Debug, Deserialize)]
struct TagCategories {
    category: HashMap<String, Vec<CategoryRegex>>,
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
                if value
                       .iter()
                       .any(|v| v.re.is_match(&transaction.description)) {
                    transaction.tags.push(key.clone());
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

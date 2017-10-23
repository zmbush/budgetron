use budgetronlib::error::BResult;
use loading::Transaction;
use processing::Collate;
use processing::regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct MoneyRange {
    low: f64,
    high: f64,
}

#[derive(Debug, Deserialize)]
struct Matchers {
    description: Option<Vec<Regex>>,
    category: Option<Vec<Regex>>,
    range: Option<MoneyRange>,
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
                        .any(|v| v.0.is_match(&transaction.original_description))
                    {
                        transaction.tags.push(key.clone());
                        continue;
                    }
                }

                if let Some(ref category) = value.category {
                    if category
                        .iter()
                        .any(|v| v.0.is_match(&transaction.original_category))
                    {
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

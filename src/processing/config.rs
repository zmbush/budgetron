use processing::Collate;
use std::collections::HashMap;
use budgetronlib::error::BResult;
use loading::{Money, Transaction};
use processing::regex::Regex;

#[derive(Debug, Deserialize)]
pub struct ConfiguredProcessors {
    processor: Vec<Processor>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Processor {
    Categorize {
        categories: HashMap<String, Vec<String>>,
    },
    AssignOwners {
        owners: HashMap<String, TransactionMatcher>,
    },
    AddTags {
        tags: HashMap<String, TransactionMatcher>,
    },
}

#[derive(Debug, Deserialize)]
pub struct TransactionMatcher {
    account:     Option<Vec<Regex>>,
    description: Option<Vec<Regex>>,
    category:    Option<Vec<Regex>>,
    range:       Option<MoneyRange>,
}

impl TransactionMatcher {
    fn matches(&self, t: &Transaction) -> bool {
        if let Some(ref description) = self.description {
            if description
                .iter()
                .any(|v| v.is_match(&t.original_description))
            {
                return true;
            }
        }
        if let Some(ref category) = self.category {
            if category.iter().any(|v| v.is_match(&t.original_category)) {
                return true;
            }
        }
        if let Some(ref account) = self.account {
            if account.iter().any(|v| v.is_match(&t.account_name)) {
                return true;
            }
        }
        if let Some(ref range) = self.range {
            if t.amount >= range.low && t.amount < range.high {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Deserialize)]
pub struct MoneyRange {
    low:  Money,
    high: Money,
}


impl Collate for ConfiguredProcessors {
    fn collate(&self, mut transactions: Vec<Transaction>) -> BResult<Vec<Transaction>> {
        for p in &self.processor {
            transactions = p.collate(transactions)?;
        }
        Ok(transactions)
    }
}

impl Collate for Processor {
    fn collate(&self, mut transactions: Vec<Transaction>) -> BResult<Vec<Transaction>> {
        use self::Processor::*;
        match *self {
            Categorize { ref categories } => for transaction in &mut transactions {
                let cat = &transaction.original_category;
                for (key, values) in categories {
                    if key == cat || (values.len() > 0 && values.contains(&cat.to_owned())) {
                        transaction.category = key.clone();
                    }
                }
            },
            AssignOwners { ref owners } => for transaction in &mut transactions {
                for (owner, matcher) in owners {
                    if matcher.matches(transaction) {
                        transaction.person = owner.clone();
                    }
                }
            },
            AddTags { ref tags } => for transaction in &mut transactions {
                for (tag, matcher) in tags {
                    if matcher.matches(transaction) {
                        transaction.tags.push(tag.to_owned());
                    }
                }
            },
        }
        Ok(transactions)
    }
}

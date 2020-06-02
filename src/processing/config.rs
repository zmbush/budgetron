// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    crate::{
        loading::{Money, Transaction},
        processing::{regex::Regex, Collate, RefundCollator, TransferCollator},
    },
    budgetronlib::{error::BResult, fintime::Date},
    serde::Deserialize,
    std::collections::HashMap,
};

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
    OverrideOwners {
        owner_override: Regex,
    },
    AddTags {
        tags: HashMap<String, TransactionMatcher>,
    },
    OwnersForTag {
        tag_owner: HashMap<String, String>,
    },
    HideAccount {
        hide_accounts: Vec<String>,
    },
    HideDescription {
        hide_description: Vec<Regex>,
    },
    Transfers {
        transfer_horizon: usize,
    },
    Refunds {
        refund_horizon: usize,
    },
}

#[derive(Debug, Deserialize)]
pub struct TransactionMatcher {
    account: Option<Vec<Regex>>,
    description: Option<Vec<Regex>>,
    category: Option<Vec<Regex>>,
    note: Option<Vec<Regex>>,
    range: Option<MoneyRange>,

    only_before: Option<Date>,
    only_after: Option<Date>,
}

impl TransactionMatcher {
    fn matches(&self, t: &Transaction) -> bool {
        if let Some(only_before) = self.only_before {
            if t.date > only_before {
                return false;
            }
        }

        if let Some(only_after) = self.only_after {
            if t.date < only_after {
                return false;
            }
        }

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
        if let Some(ref note) = self.note {
            if note.iter().any(|v| v.is_match(&t.notes)) {
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
    low: Money,
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
            Categorize { ref categories } => {
                for transaction in &mut transactions {
                    let cat = &transaction.original_category;
                    for (key, values) in categories {
                        if key == cat || (!values.is_empty() && values.contains(&cat.to_owned())) {
                            transaction.category = key.clone();
                        }
                    }
                }
            }
            AssignOwners { ref owners } => {
                for transaction in &mut transactions {
                    for (owner, matcher) in owners {
                        if matcher.matches(transaction) {
                            transaction.person = owner.clone();
                        }
                    }
                }
            }
            OverrideOwners { ref owner_override } => {
                for transaction in &mut transactions {
                    if let Some(captures) = owner_override.captures(&transaction.notes) {
                        if let Some(new_owner) = captures.get(1) {
                            transaction.person = new_owner.as_str().to_owned();
                        }
                    }
                }
            }
            AddTags { ref tags } => {
                for transaction in &mut transactions {
                    for (tag, matcher) in tags {
                        if matcher.matches(transaction) {
                            transaction.tags.push(tag.to_owned());
                        }
                    }
                }
            }
            OwnersForTag { ref tag_owner } => {
                for transaction in &mut transactions {
                    for (tag, owner) in tag_owner {
                        if transaction.tags.contains(tag) {
                            transaction.person = owner.to_owned();
                        }
                    }
                }
            }
            HideAccount { ref hide_accounts } => {
                transactions.retain(|t| !hide_accounts.contains(&t.account_name))
            }
            HideDescription {
                ref hide_description,
            } => transactions.retain(|t| {
                for d in hide_description {
                    if d.is_match(&t.description) {
                        return false;
                    }
                }
                true
            }),
            Transfers { transfer_horizon } => {
                transactions = TransferCollator::new(transfer_horizon).collate(transactions)?;
            }
            Refunds { refund_horizon } => {
                transactions = RefundCollator::new(refund_horizon).collate(transactions)?;
            }
        }
        Ok(transactions)
    }
}

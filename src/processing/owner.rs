use budgetronlib::error::BResult;
use loading::Transaction;
use processing::Collate;
use processing::regex::Regex;
use std::collections::HashMap;

#[derive(Deserialize)]
struct OwnerInfo {
    accounts: Option<Vec<Regex>>,
    categories: Option<Vec<Regex>>,
    descriptions: Option<Vec<Regex>>,
}

#[derive(Deserialize)]
pub struct OwnersConfig {
    owners: HashMap<String, OwnerInfo>,
}

pub struct OwnersCollator {
    config: OwnersConfig,
}

impl OwnersCollator {
    pub fn new(config: OwnersConfig) -> OwnersCollator {
        OwnersCollator { config }
    }
}
impl Collate for OwnersCollator {
    fn collate(&self, transactions: Vec<Transaction>) -> BResult<Vec<Transaction>> {
        Ok(
            transactions
                .into_iter()
                .map(|mut transaction| {
                    // Check regexes
                    for (ref owner, ref info) in &self.config.owners {
                        if let Some(ref accounts) = info.accounts {
                            for account_re in accounts {
                                if account_re.0.is_match(&transaction.account_name) {
                                    transaction.person = (*owner).clone();
                                    return transaction;
                                }
                            }
                        }

                        if let Some(ref categories) = info.categories {
                            for category_re in categories {
                                if category_re.0.is_match(&transaction.original_category) {
                                    transaction.person = (*owner).clone();
                                    return transaction;
                                }
                            }
                        }

                        if let Some(ref descriptions) = info.descriptions {
                            for description_re in descriptions {
                                if description_re.0.is_match(&transaction.description) {
                                    transaction.person = (*owner).clone();
                                    return transaction;
                                }
                            }
                        }
                    }

                    transaction
                })
                .collect(),
        )
    }
}

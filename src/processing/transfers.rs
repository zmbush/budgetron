use budgetronlib::error::BResult;
use loading::{Transaction, TransactionType};
use processing::Collate;
use std::cmp::min;
use std::collections::{HashMap, HashSet};

pub struct TransferCollator {
    pub horizon: usize,
}

impl TransferCollator {
    pub fn new(horizon: usize) -> TransferCollator {
        TransferCollator { horizon }
    }
}

impl Collate for TransferCollator {
    fn collate(&self, mut transactions: Vec<Transaction>) -> BResult<Vec<Transaction>> {
        let mut to_delete = HashSet::new();
        let mut to_update = HashMap::new();
        for (i, t) in transactions.iter().enumerate() {
            for j in i..min(transactions.len(), i + self.horizon) {
                let ref tn = transactions[j];
                if tn.amount == t.amount && tn.transaction_type != t.transaction_type &&
                    !to_delete.contains(&i) && !to_delete.contains(&j) &&
                    !to_update.contains_key(&i) && !to_update.contains_key(&j)
                {
                    if t.account_name == tn.account_name {
                        to_delete.insert(i);
                        to_delete.insert(j);
                    } else {
                        match t.transaction_type {
                            TransactionType::Debit => {
                                to_delete.insert(j);
                                to_update.insert(i, tn.account_name.clone());
                            },
                            TransactionType::Credit => {
                                to_delete.insert(i);
                                to_update.insert(j, t.account_name.clone());
                            },
                            TransactionType::Transfer => unreachable!(),
                        };
                    }
                }
            }
        }

        for (i, destination_account) in to_update {
            if let Some(transaction) = transactions.get_mut(i) {
                transaction.transfer_destination_account = Some(destination_account);
                transaction.transaction_type = TransactionType::Transfer;
            }
        }

        let mut to_delete: Vec<_> = to_delete.into_iter().collect();
        to_delete.sort();
        to_delete.reverse();

        for i in to_delete {
            transactions.remove(i);
        }
        Ok(transactions)
    }
}

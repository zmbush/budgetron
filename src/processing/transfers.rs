// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::loading::{Transaction, TransactionType};
use crate::processing::Collate;
use budgetronlib::error::BResult;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::i64;

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
            loop {
                let candidates: Vec<_> = (i..min(transactions.len(), i + self.horizon))
                    .filter_map(|j| {
                        let tn = &transactions[j];
                        if tn.amount == t.amount
                            && !to_delete.contains(&i)
                            && !to_delete.contains(&j)
                            && !to_update.contains_key(&i)
                            && !to_update.contains_key(&j)
                        {
                            Some(j)
                        } else {
                            None
                        }
                    })
                    .collect();

                if candidates.len() <= 1 {
                    break;
                }

                let mut mindelta = i64::MAX;
                let mut found_transfer = (0, 0);
                let debits = candidates
                    .iter()
                    .filter(|&i| transactions[*i].transaction_type.is_debit());

                for debit_ix in debits {
                    let debit = &transactions[*debit_ix];
                    let credits = candidates
                        .iter()
                        .filter(|&i| transactions[*i].transaction_type.is_credit());
                    for credit_ix in credits {
                        let credit = &transactions[*credit_ix];
                        if (debit.date - credit.date).abs() < mindelta
                            && debit.account_name != credit.account_name
                        {
                            found_transfer = (*debit_ix, *credit_ix);
                            mindelta = (debit.date - credit.date).abs();
                        }
                    }
                }

                if found_transfer != (0, 0) {
                    let tn = &transactions[found_transfer.1];

                    to_delete.insert(found_transfer.1);
                    to_update.insert(found_transfer.0, tn.account_name.clone());

                    if found_transfer.0 == i || found_transfer.1 == i {
                        break;
                    }
                } else {
                    break;
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

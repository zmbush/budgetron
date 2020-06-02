// Copyright 2018 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    crate::{loading::Transaction, processing::Collate},
    budgetronlib::error::BResult,
    std::{cmp::min, collections::HashSet, i64},
};

pub struct RefundCollator {
    pub horizon: usize,
}

impl RefundCollator {
    pub fn new(horizon: usize) -> RefundCollator {
        RefundCollator { horizon }
    }
}

impl Collate for RefundCollator {
    fn collate(&self, mut transactions: Vec<Transaction>) -> BResult<Vec<Transaction>> {
        let mut to_delete = HashSet::new();
        for (i, t) in transactions.iter().enumerate() {
            loop {
                let candidates: Vec<_> = (i..min(transactions.len(), i + self.horizon))
                    .filter(|&j| {
                        let tn = &transactions[j];
                        tn.amount == t.amount && !to_delete.contains(&i) && !to_delete.contains(&j)
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
                            && debit.account_name == credit.account_name
                        {
                            found_transfer = (*debit_ix, *credit_ix);
                            mindelta = (debit.date - credit.date).abs();
                        }
                    }
                }

                if found_transfer != (0, 0) {
                    to_delete.insert(found_transfer.1);
                    to_delete.insert(found_transfer.0);

                    if found_transfer.0 == i || found_transfer.1 == i {
                        break;
                    }
                } else {
                    break;
                }
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

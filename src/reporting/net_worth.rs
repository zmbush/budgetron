use loading::{Transaction, TransactionType};
use reporting::Reporter;
use std::collections::HashMap;

pub struct NetWorth;
#[derive(Debug)]
pub struct NetWorthReport {
    pub worth: HashMap<String, f64>,
}

impl Reporter for NetWorth {
    type OutputType = NetWorthReport;

    fn report(&self, transactions: &Vec<Transaction>) -> NetWorthReport {
        let mut worth = HashMap::new();
        for transaction in transactions {
            *worth.entry(transaction.account_name.clone()).or_insert(0.0) +=
                match transaction.transaction_type {
                    TransactionType::Credit => transaction.amount,
                    TransactionType::Debit => -transaction.amount,
                    TransactionType::Transfer => -transaction.amount,
                };
            if let TransactionType::Transfer = transaction.transaction_type {
                *worth
                     .entry(transaction.transfer_destination_account.clone()
                         .expect("transfer records should have a transfer_destination_account"))
                     .or_insert(0.0) += transaction.amount;
            }
        }
        NetWorthReport { worth }
    }
}

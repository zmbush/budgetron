use loading::{Transaction, TransactionType};
use reporting::Reporter;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;

pub struct NetWorth;
#[derive(Debug, Serialize)]
pub struct NetWorthReport {
    pub worth: BTreeMap<String, f64>,
}

impl Reporter for NetWorth {
    type OutputType = NetWorthReport;

    fn report<'a, I>(&self, transactions: I) -> NetWorthReport
        where I: Iterator<Item = Cow<'a, Transaction>>
    {
        let mut worth = BTreeMap::new();
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

impl NetWorthReport {
    pub fn print(&self) {
        println!("{}", self);
    }
}

impl fmt::Display for NetWorthReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let maxlen = self.worth.keys().map(String::len).max().unwrap_or(0);
        writeln!(f, "{:>width$}  {}", "Account", "Balance", width = maxlen)?;
        for (key, value) in &self.worth {
            if *value < 0.001 && *value > -0.001 {
                continue;
            }
            writeln!(f, "{:>width$}  {:.2}", key, value, width = maxlen)?;
        }

        Ok(())
    }
}

use budgetronlib::config::CategoryConfig;
use budgetronlib::error::BResult;
use budgetronlib::fintime::Date;
use loading::generic::{Genericize, Transaction, TransactionType};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
struct LogixMoney {
    amount: f64,
}

#[derive(Debug)]
struct LogixTransactionAmount {
    amount: LogixMoney,
    negative: bool,
}

#[derive(Debug, Deserialize)]
pub struct LogixExport {
    account: String,
    date: Date,
    amount: LogixTransactionAmount,
    balance: LogixTransactionAmount,
    category: String,
    description: String,
    memo: String,
    notes: String,
}

impl FromStr for LogixMoney {
    type Err = String;
    fn from_str(s: &str) -> Result<LogixMoney, String> {
        if s.starts_with("$") {
            Ok(LogixMoney {
                amount: if let Ok(amt) = s[1..].parse() {
                    amt
                } else {
                    return Err(format!("unable to parse number'{} '", s));
                },
            })
        } else {
            Err(format!("'{}' does not look like money", s))
        }
    }
}

struct LogixTransactionAmountVisitor;
impl<'de> Visitor<'de> for LogixTransactionAmountVisitor {
    type Value = LogixTransactionAmount;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a money amount")
    }

    fn visit_str<E: de::Error>(self, s: &str) -> Result<LogixTransactionAmount, E> {
        let negative = s.starts_with("(") && s.ends_with(")");
        Ok(LogixTransactionAmount {
            amount: if negative { s[1..s.len() - 1].parse() } else { s.parse() }.map_err(
                |e| E::custom(e),
            )?,
            negative,
        })
    }
}

impl<'de> Deserialize<'de> for LogixTransactionAmount {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_str(LogixTransactionAmountVisitor)
    }
}

impl Genericize for LogixExport {
    fn genericize(self, cfg: &CategoryConfig) -> BResult<Transaction> {
        Ok(Transaction {
            date: self.date,
            person: "".to_owned(),
            description: self.description.clone(),
            original_description: self.description,
            amount: self.amount.amount.amount,
            transaction_type: if self.amount.negative {
                TransactionType::Debit
            } else {
                TransactionType::Credit
            },
            category: cfg.find_category(&self.category)?.to_owned(),
            original_category: self.category,
            account_name: self.account,
            labels: self.memo,
            notes: self.notes,
            transfer_destination_account: None,
            tags: vec![],
        })
    }
}

use budgetronlib::config;
use budgetronlib::fintime::Date;
use budgetronlib::error::BResult;
use serde::de::{self, Visitor, Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Serialize)]
pub enum TransactionType {
    Credit,
    Debit,
}

#[derive(Debug, Serialize)]
pub enum Person {
    Molly,
    Zach,
}

#[derive(Debug, Serialize)]
pub struct Transaction {
    pub date: Date,
    pub person: Person,
    pub description: String,
    pub original_description: String,
    pub amount: f64,
    pub transaction_type: TransactionType,
    pub category: String,
    pub original_category: String,
    pub account_name: String,
    pub labels: String,
    pub notes: String,
}

pub trait Genericize {
    fn genericize(self, &config::CategoryConfig) -> BResult<Transaction>;
}

struct TransactionTypeVisitor;
impl<'de> Visitor<'de> for TransactionTypeVisitor {
    type Value = TransactionType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string in the set [debit, credit]")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<TransactionType, E> {
        match value {
            "debit" => Ok(TransactionType::Debit),
            "credit" => Ok(TransactionType::Credit),
            s => Err(E::custom(format!("'{}' is not one of credit or debit", s)))
        }
    }
}

impl<'de> Deserialize<'de> for TransactionType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_str(TransactionTypeVisitor)
    }
}

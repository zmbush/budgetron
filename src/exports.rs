use common::{Genericize, Person, Transaction, TransactionType};
use fintime::Date;
use std::str::FromStr;
use serde::{de, Deserialize, Deserializer};
use rustc_serialize::{Decodable, Decoder};
use config;
use std::fmt;

#[derive(Debug)]
struct TransactionAmount {
    amount: Money,
    negative: bool,
}

impl FromStr for TransactionAmount {
    type Err = String;
    fn from_str(s: &str) -> Result<TransactionAmount, String> {
        let negative = s.starts_with("(") && s.ends_with(")");
        Ok(TransactionAmount {
            amount: try!(if negative {
                s[1..s.len() - 1].parse()
            } else {
                s.parse()
            }),
            negative: negative,
        })
    }
}

struct TransactionAmountVisitor;
impl de::Visitor for TransactionAmountVisitor {
    type Value = TransactionAmount;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a money amount")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<TransactionAmount, E> {
        match value.parse() {
            Ok(m) => Ok(m),
            Err(s) => Err(E::custom(s))
        }
    }
}

impl Deserialize for TransactionAmount {
    fn deserialize<D: Deserializer>(d: D) -> Result<Self, D::Error> {
        d.deserialize_str(TransactionAmountVisitor)
    }
}

impl Decodable for TransactionAmount {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        let s = d.read_str()?;
        match s.parse() {
            Ok(m) => Ok(m),
            Err(s) => Err(d.error(&s))
        }
    }
}

#[derive(Debug)]
struct Money {
    amount: f64,
}

impl FromStr for Money {
    type Err = String;
    fn from_str(s: &str) -> Result<Money, String> {
        if s.starts_with("$") {
            if let Ok(amt) = s[1..].parse() {
                Ok(Money { amount: amt })
            } else {
                Err("Unable to parse number".to_owned())
            }
        } else {
            Err(format!("'{}' does not look like money", s))
        }
    }
}

struct MoneyVisitor;
impl de::Visitor for MoneyVisitor {
    type Value = Money;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("money")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Money, E> {
        match value.parse() {
            Ok(m) => Ok(m),
            Err(s) => Err(E::custom(s))
        }
    }
}
impl Deserialize for Money {
    fn deserialize<D: Deserializer>(d: D) -> Result<Self, D::Error> {
        d.deserialize_str(MoneyVisitor)
    }
}
impl Decodable for Money{
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        let s = try!(d.read_str());
        match s.parse() {
            Ok(m) => Ok(m),
            Err(s) => Err(d.error(&s)),
        }
    }
}

#[derive(Deserialize, Debug, RustcDecodable)]
pub struct LogixExport {
    account: String,
    date: Date,
    amount: TransactionAmount,
    balance: TransactionAmount,
    category: String,
    description: String,
    memo: String,
    notes: String,
}

impl Genericize for LogixExport {
    fn genericize(self, cfg: &config::CategoryConfig) -> Transaction {
        Transaction {
            date: self.date,
            person: Person::Molly,
            description: self.description.clone(),
            original_description: self.description,
            amount: self.amount.amount.amount,
            transaction_type: if self.amount.negative {
                TransactionType::Debit
            } else {
                TransactionType::Credit
            },
            category: cfg.find_category(&self.category).unwrap().to_owned(),
            original_category: self.category,
            account_name: self.account,
            labels: self.memo,
            notes: self.notes,
        }
    }
}

#[derive(Deserialize, Debug, RustcDecodable)]
pub struct MintExport {
    date: Date,
    description: String,
    original_description: String,
    amount: f64,
    transaction_type: TransactionType,
    category: String,
    account_name: String,
    labels: String,
    notes: String,
}

impl Genericize for MintExport {
    fn genericize(self, cfg: &config::CategoryConfig) -> Transaction {
        Transaction {
            date: self.date,
            person: Person::Zach,
            description: self.description,
            original_description: self.original_description,
            amount: self.amount,
            transaction_type: self.transaction_type,
            category: cfg.find_category(&self.category).unwrap().to_owned(),
            original_category: self.category,
            account_name: self.account_name,
            labels: self.labels,
            notes: self.notes,
        }
    }
}

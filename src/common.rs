use csv;
use error::BResult;
use rustc_serialize::{Decodable, Decoder};
use std::cmp::min;
use fintime::Date;
use std::collections::HashSet;
use config;
use data_store;

#[derive(Debug, RustcEncodable, PartialEq)]
pub enum TransactionType {
    Credit,
    Debit,
}

impl Decodable for TransactionType {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        match try!(d.read_str()).as_ref() {
            "debit" => Ok(TransactionType::Debit),
            "credit" => Ok(TransactionType::Credit),
            s => Err(d.error(&format!("'{}' is not one of debit or credit", s))),
        }
    }
}

pub trait Genericize {
    fn genericize(self, &config::CategoryConfig) -> Transaction;
}

#[derive(Debug, RustcEncodable)]
pub enum Person {
    Molly,
    Zach,
}

#[derive(Debug, RustcEncodable)]
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

pub struct Transactions<'a> {
    pub transactions: Vec<Transaction>,
    config: &'a config::CategoryConfig,
}

impl<'a> Transactions<'a> {
    pub fn new(config: &'a config::CategoryConfig) -> Transactions<'a> {
        Transactions {
            transactions: Vec::new(),
            config: config,
        }
    }

    pub fn load_records<ExportType>(&mut self, filename: &str) -> BResult<i32>
        where ExportType: Genericize + Decodable
    {
        let mut count = 0;
        for record in try!(csv::Reader::from_file(filename)).decode() {
            let record: ExportType = try!(record);
            self.transactions.push(record.genericize(&self.config));
            count += 1;
        }
        Ok(count)
    }

    pub fn collate(&mut self) {
        self.transactions.sort_by(|a, b| a.date.cmp(&b.date));

        let mut to_delete = HashSet::new();

        for (i, t) in self.transactions.iter().enumerate() {
            if self.config.ignored_accounts.contains(&t.account_name) {
                to_delete.insert(i);
            }
        }

        for (i, t) in self.transactions.iter().enumerate() {
            for j in i..min(self.transactions.len(), i + 100) {
                let ref tn = self.transactions[j];
                if tn.amount == t.amount && tn.transaction_type != t.transaction_type &&
                   !to_delete.contains(&i) && !to_delete.contains(&j) {
                    to_delete.insert(i);
                    to_delete.insert(j);
                }
            }
        }

        let mut to_delete: Vec<_> = to_delete.into_iter().collect();
        to_delete.sort();
        to_delete.reverse();

        for i in to_delete {
            self.transactions.remove(i);
        }
    }

    pub fn write_to_db(&self, db: &data_store::Transactions) {
        let mut all_transactions = Vec::new();
        for t in self.iter() {
            all_transactions.push(data_store::models::NewTransaction {
                date: t.date.date.naive_utc(),
                person: match t.person {
                    Person::Molly => "Molly",
                    Person::Zach => "Zach",
                },
                description: &t.description,
                original_description: &t.original_description,
                amount: t.amount,
                transaction_type: match t.transaction_type {
                    TransactionType::Debit => "Debit",
                    TransactionType::Credit => "Credit",
                },
                category: &t.category,
                original_category: &t.original_category,
                account_name: &t.account_name,
                labels: &t.labels,
                notes: &t.notes,
            });
        }
        if !all_transactions.is_empty() {
            db.set_transactions(&all_transactions);
        }
    }

    pub fn iter(&self) -> ::std::slice::Iter<Transaction> {
        self.transactions.iter()
    }

    pub fn date_of_last_transaction(&self) -> Option<Date> {
        self.transactions.last().map(|t| t.date)
    }
}

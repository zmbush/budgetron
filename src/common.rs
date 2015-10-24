use rustc_serialize::{Decoder, Decodable, Encoder, Encodable};
use error::BResult;
use csv;
use std::collections::HashMap;
use std::cmp::min;
use time;
use std::fmt;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32
}

impl Date {
    pub fn from_tm(n: time::Tm) -> Date {
        Date {
            year: n.tm_year + 1900,
            month: n.tm_mon + 1,
            day: n.tm_mday,
        }
    }

    pub fn today() -> Date {
        Date::ago("0d")
    }

    pub fn ago(time_frame: &str) -> Date {
        let mut tf = time_frame.to_owned();
        let mut tm = time::now() - time::Duration::weeks(1);
        if let Some(c) = tf.pop() {
            let num = tf.parse().unwrap();
            tm = tm - match c {
                'w' => time::Duration::weeks(num),
                'd' => time::Duration::days(num),
                'm' => time::Duration::days(num * 28),
                'q' => time::Duration::weeks(num * 13),
                'y' => time::Duration::days(num * 365),
                _ => time::Duration::days(0)
            };
        }
        Date::from_tm(tm)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:04}{:02}{:02}", self.year, self.month, self.day)
        } else {
            write!(f, "{}/{}/{}", self.month, self.day, self.year)
        }
    }
}

impl Decodable for Date {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        let s = try!(d.read_str());
        let error = Err(d.error(&format!("Bad date format '{}'", s)));

        macro_rules! get_num {
            ($d:ident) => {
                match $d.next() {
                    Some(s) => match s.parse() {
                        Ok(i) => i,
                        Err(_) => return error
                    },
                    None => return error
                }
            }
        }

        let mut parts = s.split("/");
        Ok(Date {
            month: get_num!(parts),
            day: get_num!(parts),
            year: get_num!(parts)
        })
    }
}

impl Encodable for Date {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_str(&format!("{}", self))
    }
}

#[derive(Debug, RustcEncodable, PartialEq)]
pub enum TransactionType { Credit, Debit }

impl Decodable for TransactionType {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        match try!(d.read_str()).as_ref() {
            "debit" => Ok(TransactionType::Debit),
            "credit" => Ok(TransactionType::Credit),
            s => Err(d.error(&format!("'{}' is not one of debit or credit", s)))
        }
    }
}

pub trait Genericize {
    fn genericize(self) -> Transaction;
}

#[derive(Debug, RustcEncodable)]
pub enum Person { Barry, Zach }

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
    pub notes: String
}

pub struct Transactions {
    transactions: Vec<Transaction>
}

impl Transactions {
    pub fn new() -> Transactions {
        Transactions {
            transactions: Vec::new()
        }
    }

    pub fn load_records<ExportType>(&mut self, filename: &str) -> BResult<i32>
            where ExportType: Genericize + Decodable {
        let mut count = 0;
        for record in try!(csv::Reader::from_file(filename)).decode() {
            let record: ExportType = try!(record);
            self.transactions.push(record.genericize());
            count += 1;
        }
        Ok(count)
    }

    pub fn collate(&mut self) {
        self.transactions.sort_by(|a, b| a.date.cmp(&b.date));

        let mut to_delete = Vec::new();
        for (i, t) in self.transactions.iter().enumerate() {
            for j in i..min(self.transactions.len(), i + 50) {
                let ref tn = self.transactions[j];
                if tn.amount == t.amount && tn.transaction_type != t.transaction_type {
                    to_delete.push(i);
                    to_delete.push(j);
                }
            }
        }

        to_delete.sort();
        to_delete.reverse();

        for i in to_delete {
            self.transactions.remove(i);
        }
    }

    pub fn iter(&self) -> ::std::slice::Iter<Transaction> {
        self.transactions.iter()
    }
}

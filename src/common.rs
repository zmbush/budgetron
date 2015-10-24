use rustc_serialize::{Decoder, Decodable, Encoder, Encodable};
use error::BResult;
use csv;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Date {
    pub year: i16,
    pub month: i8,
    pub day: i8
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
        s.emit_str(&format!("{}/{}/{}", self.month, self.day, self.year))
    }
}

#[derive(Debug, RustcEncodable)]
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

    pub fn sort(&mut self) {
        self.transactions.sort_by(|a, b| a.date.cmp(&b.date));
    }

    pub fn iter(&self) -> ::std::slice::Iter<Transaction> {
        self.transactions.iter()
    }
}

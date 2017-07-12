use budgetronlib::config;
use budgetronlib::error::BResult;
use budgetronlib::fintime::Date;

#[derive(Debug, Serialize, Copy, Deserialize, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum TransactionType {
    Credit,
    Debit,
    Transfer,
}

impl Default for TransactionType {
    fn default() -> TransactionType {
        TransactionType::Credit
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Person {
    Barry,
    Zach,
}

impl Default for Person {
    fn default() -> Person {
        Person::Barry
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Transaction {
    pub date: Date,
    pub description: String,
    pub amount: f64,
    pub transaction_type: TransactionType,
    pub person: Person,
    pub original_description: String,
    pub category: String,
    pub original_category: String,
    pub account_name: String,
    pub labels: String,
    pub notes: String,
    pub transfer_destination_account: Option<String>,
    pub tags: Vec<String>,
}

pub trait Genericize {
    fn genericize(self, &config::CategoryConfig) -> BResult<Transaction>;
}

impl Genericize for Transaction {
    fn genericize(self, _: &config::CategoryConfig) -> BResult<Transaction> {
        Ok(self)
    }
}

use budgetronlib::config;
use budgetronlib::error::BResult;
use budgetronlib::fintime::Date;

#[derive(Debug, Serialize)]
pub enum TransactionType {
    Credit,
    Debit,
    Transfer,
}

#[derive(Debug, Serialize)]
pub enum Person {
    Molly,
    Zach,
}

#[derive(Debug, Serialize)]
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
}

pub trait Genericize {
    fn genericize(self, &config::CategoryConfig) -> BResult<Transaction>;
}

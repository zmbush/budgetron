use budgetronlib::config;
use budgetronlib::error::BResult;
use budgetronlib::fintime::Date;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum TransactionType {
    Credit,
    Debit,
}

#[derive(Debug, Serialize)]
pub enum Person {
    Barry,
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

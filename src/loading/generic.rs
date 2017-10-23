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

impl TransactionType {
    pub fn is_credit(&self) -> bool {
        TransactionType::Credit == *self
    }

    pub fn is_debit(&self) -> bool {
        TransactionType::Debit == *self
    }

    pub fn is_transfer(&self) -> bool {
        TransactionType::Transfer == *self
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Transaction {
    pub date: Date,
    pub description: String,
    pub amount: f64,
    pub transaction_type: TransactionType,
    pub person: String,
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
    fn genericize(self) -> BResult<Transaction>;
}

impl Genericize for Transaction {
    fn genericize(self) -> BResult<Transaction> {
        Ok(self)
    }
}

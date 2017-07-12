use budgetronlib::config::CategoryConfig;
use budgetronlib::error::BResult;
use budgetronlib::fintime::Date;
use loading::generic::{TransactionType, Genericize, Transaction, Person};

// "id","account_id","reference_id","transaction_type","amount","posted_at",
// "created_at","nickname","original_name","merchant_id","updated_at",
// "check_number","account_name","tags"
#[derive(Debug, Deserialize)]
pub enum AlliantTransactionType {
    Debit,
    Credit,
}

impl Into<TransactionType> for AlliantTransactionType {
    fn into(self) -> TransactionType {
        match self {
            AlliantTransactionType::Debit => TransactionType::Debit,
            AlliantTransactionType::Credit => TransactionType::Credit,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AlliantExport {
    id: String,
    account_id: i64,
    reference_id: i64,
    transaction_type: AlliantTransactionType,
    amount: f64,
    posted_at: Date,
    created_at: Date,
    nickname: String,
    original_name: String,
    merchant_id: String,
    updated_at: Date,
    check_number: Option<i32>,
    account_name: String,
    tags: String,
}

impl Genericize for AlliantExport {
    fn genericize(self, cfg: &CategoryConfig) -> BResult<Transaction> {
        Ok(Transaction {
               date: self.posted_at,
               person: Person::Molly,
               description: self.nickname,
               original_description: self.original_name,
               amount: self.amount,
               transaction_type: self.transaction_type.into(),
               category: cfg.find_category(&self.tags)?.to_owned(),
               original_category: self.tags,
               account_name: self.account_name,
               labels: "".to_owned(),
               notes: "".to_owned(),
               transfer_destination_account: None,
               tags: vec![],
           })
    }
}

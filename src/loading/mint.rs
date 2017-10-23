use budgetronlib::error::BResult;
use budgetronlib::fintime::Date;
use loading::generic::{Genericize, Transaction, TransactionType};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MintTransactionType {
    Debit,
    Credit,
}

impl Into<TransactionType> for MintTransactionType {
    fn into(self) -> TransactionType {
        match self {
            MintTransactionType::Debit => TransactionType::Debit,
            MintTransactionType::Credit => TransactionType::Credit,
        }
    }
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MintExport {
    date: Date,
    description: String,
    #[serde(rename = "Original Description")] original_description: String,
    amount: f64,
    #[serde(rename = "Transaction Type")] transaction_type: MintTransactionType,
    category: String,
    #[serde(rename = "Account Name")] account_name: String,
    labels: String,
    notes: String,
}

impl Genericize for MintExport {
    fn genericize(self) -> BResult<Transaction> {
        Ok(Transaction {
            date: self.date,
            person: "".to_owned(),
            description: self.description,
            original_description: self.original_description,
            amount: self.amount,
            transaction_type: self.transaction_type.into(),
            category: self.category.clone(),
            original_category: self.category,
            account_name: self.account_name,
            labels: self.labels,
            notes: self.notes,
            transfer_destination_account: None,
            tags: vec![],
        })
    }
}

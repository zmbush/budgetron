use budgetronlib::fintime::Date;
use budgetronlib::error::BResult;
use budgetronlib::config::CategoryConfig;
use generic::{Transaction, Person, TransactionType, Genericize};

#[derive(Debug, Deserialize)]
pub struct MintExport {
    date: Date,
    description: String,
    original_description: String,
    amount: f64,
    transaction_type: TransactionType,
    category: String,
    account_name: String,
    labels: String,
    notes: String
}

impl Genericize for MintExport {
    fn genericize(self, cfg: &CategoryConfig) -> BResult<Transaction> {
        Ok(Transaction {
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
        })

    }
}

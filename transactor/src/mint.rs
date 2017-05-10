use budgetronlib::config::CategoryConfig;
use budgetronlib::error::BResult;
use budgetronlib::fintime::Date;
use generic::{Transaction, Person, TransactionType, Genericize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MintExport {
    date: Date,
    description: String,
    #[serde(rename="Original Description")]
    original_description: String,
    amount: f64,
    #[serde(rename="Transaction Type")]
    transaction_type: TransactionType,
    category: String,
    #[serde(rename="Account Name")]
    account_name: String,
    labels: String,
    notes: String,
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

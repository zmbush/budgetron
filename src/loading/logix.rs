use budgetronlib::error::BResult;
use budgetronlib::fintime::Date;
use loading::generic::{Genericize, Transaction, TransactionType};
use loading::money::Money;

#[derive(Debug, Deserialize)]
pub struct LogixExport {
    account:     String,
    date:        Date,
    amount:      Money,
    balance:     Money,
    category:    String,
    description: String,
    memo:        String,
    notes:       String,
}

impl Genericize for LogixExport {
    fn genericize(self) -> BResult<Transaction> {
        Ok(Transaction {
            date: self.date,
            person: "".to_owned(),
            description: self.description.clone(),
            original_description: self.description,
            amount: self.amount.abs(),
            transaction_type: if self.amount.is_negative() {
                TransactionType::Debit
            } else {
                TransactionType::Credit
            },
            category: self.category.clone(),
            original_category: self.category,
            account_name: self.account,
            labels: self.memo,
            notes: self.notes,
            transfer_destination_account: None,
            tags: vec![],
        })
    }
}

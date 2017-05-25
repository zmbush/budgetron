use data_store;
use loading::{Person, TransactionType};
use loading::Transaction;
use reporting::Reporter;

pub struct Database;
impl Reporter for Database {
    type OutputType = ();

    fn report(&self, transactions: &Vec<Transaction>) {
        let db = data_store::Transactions::new_from_env();
        let mut all_transactions = Vec::new();
        for t in transactions {
            all_transactions.push(data_store::models::NewTransaction {
                                      date: t.date.date.naive_utc(),
                                      person: match t.person {
                                          Person::Barry => "Barry",
                                          Person::Zach => "Zach",
                                      },
                                      description: &t.description,
                                      original_description: &t.original_description,
                                      amount: t.amount,
                                      transaction_type: match t.transaction_type {
                                          TransactionType::Debit => "Debit",
                                          TransactionType::Credit => "Credit",
                                          TransactionType::Transfer => "Transfer",
                                      },
                                      category: &t.category,
                                      original_category: &t.original_category,
                                      account_name: &t.account_name,
                                      labels: &t.labels,
                                      notes: &t.notes,
                                      transfer_destination_account: t.transfer_destination_account
                                          .as_ref(),
                                  })
        }
        if !all_transactions.is_empty() {
            db.set_transactions(all_transactions);
        }
    }
}

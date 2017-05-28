use data_store;
use loading::{Person, TransactionType};
use loading::Transaction;
use reporting::Reporter;
use std::borrow::Cow;

pub struct Database;
impl Reporter for Database {
    type OutputType = ();

    fn report<'a, I>(&self, transactions: I)
        where I: Iterator<Item = Cow<'a, Transaction>>
    {
        let db = data_store::Transactions::new_from_env();
        let mut all_transactions = Vec::new();
        for t in transactions.into_iter() {
            let t = t.into_owned();
            all_transactions.push(data_store::models::NewTransaction {
                                      date: t.date.date.naive_utc(),
                                      person: match t.person {
                                          Person::Molly => "Molly",
                                          Person::Zach => "Zach",
                                      },
                                      description: t.description,
                                      original_description: t.original_description,
                                      amount: t.amount,
                                      transaction_type: match t.transaction_type {
                                          TransactionType::Debit => "Debit",
                                          TransactionType::Credit => "Credit",
                                          TransactionType::Transfer => "Transfer",
                                      },
                                      category: t.category,
                                      original_category: t.original_category,
                                      account_name: t.account_name,
                                      labels: t.labels,
                                      notes: t.notes,
                                      transfer_destination_account: t.transfer_destination_account,
                                  })
        }
        if !all_transactions.is_empty() {
            db.set_transactions(all_transactions);
        }
    }
}

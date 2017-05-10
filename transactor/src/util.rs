use budgetronlib::config::CategoryConfig;
use budgetronlib::error::BResult;
use generic::{Transaction, Genericize};
use serde::de::DeserializeOwned;
use csv::Reader;

pub fn from_file<TransactionType>(filename: &str, config: &CategoryConfig) -> BResult<Vec<Transaction>>
where TransactionType: Genericize + DeserializeOwned {
    let mut transactions = Vec::new();
    for record in try!(Reader::from_path(filename)).deserialize() {
        let record: TransactionType = try!(record);
        transactions.push(record.genericize(config)?);
    }
    Ok(transactions)
}

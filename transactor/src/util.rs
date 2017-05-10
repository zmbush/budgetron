use budgetronlib::config::CategoryConfig;
use budgetronlib::error::{BResult, BudgetError};
use csv::Reader;
use generic::{Transaction, Genericize};
use logix;
use mint;
use serde::de::DeserializeOwned;
use std::fs::File;

pub fn from_file<TransactionType>(filename: &str,
                                  config: &CategoryConfig)
                                  -> BResult<Vec<Transaction>>
    where TransactionType: Genericize + DeserializeOwned
{
    let mut transactions = Vec::new();
    for record in Reader::from_path(filename)?.deserialize() {
        let record: TransactionType = record?;
        transactions.push(record.genericize(config)?);
    }
    Ok(transactions)
}

pub fn from_file_inferred(filename: &str, config: &CategoryConfig) -> BResult<Vec<Transaction>> {
    File::open(filename)?;

    if let Ok(result) = from_file::<mint::MintExport>(filename, config) {
        Ok(result)
    } else if let Ok(result) = from_file::<logix::LogixExport>(filename, config) {
        Ok(result)
    } else {
        Err(BudgetError::NoMatchingImporter)
    }
}

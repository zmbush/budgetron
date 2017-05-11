use alliant;
use budgetronlib::config::CategoryConfig;
use budgetronlib::error::{BResult, BudgetError};
use csv::Reader;
use generic::{Transaction, Genericize};
use logix;
use mint;
use serde::de::DeserializeOwned;
use std::fmt::Display;
use std::fs::File;
use std::path::Path;

fn from_file<TransactionType, P>(filename: P, config: &CategoryConfig) -> BResult<Vec<Transaction>>
    where TransactionType: Genericize + DeserializeOwned,
          P: AsRef<Path>
{
    let mut transactions = Vec::new();
    for record in Reader::from_path(filename)?.deserialize() {
        let record: TransactionType = record?;
        transactions.push(record.genericize(config)?);
    }
    Ok(transactions)
}

fn from_file_inferred<P: AsRef<Path> + Copy>(filename: P,
                                             config: &CategoryConfig)
                                             -> BResult<Vec<Transaction>> {
    // If the file doesn't exist. Don't bother.
    File::open(filename)?;

    let mut errors = Vec::new();

    macro_rules! parse_exports {
        ($($type:path),*) => ($(match from_file::<$type, _>(filename, config) {
            Ok(result) => return Ok(result),
            Err(e) => errors.push(e)
        })*)
    }
    parse_exports!(mint::MintExport, logix::LogixExport, alliant::AlliantExport);
    Err(BudgetError::Multi(errors))
}

pub fn load_from_files<P: AsRef<Path> + Display, Files: Iterator<Item = P>>
    (filenames: Files,
     config: &CategoryConfig)
     -> BResult<Vec<Transaction>> {
    let mut transactions = Vec::new();
    for filename in filenames {
        info!("Opening file: {}", filename);
        transactions.append(&mut from_file_inferred(&filename, &config)?);
    }

    transactions.sort_by(|a, b| a.date.cmp(&b.date));

    Ok(transactions)
}

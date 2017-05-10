use csv;
use std::convert::From;
use toml;

#[derive(Debug)]
pub enum BudgetError {
    CSVError(csv::Error),
    NoCategoryFoundError(String),
    TomlDeError(toml::de::Error),
    NoTransactionError,
}

impl From<csv::Error> for BudgetError {
    fn from(e: csv::Error) -> BudgetError {
        BudgetError::CSVError(e)
    }
}

impl From<toml::de::Error> for BudgetError {
    fn from(e: toml::de::Error) -> BudgetError {
        BudgetError::TomlDeError(e)
    }
}

pub type BResult<T> = Result<T, BudgetError>;

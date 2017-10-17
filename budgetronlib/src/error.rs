use csv;
use std::convert::From;
use std::io;
use toml;
use serde_yaml;

#[derive(Debug)]
pub enum BudgetError {
    CSVError(csv::Error),
    NoCategoryFoundError(String),
    TomlDeError(toml::de::Error),
    SerdeYamlError(serde_yaml::Error),
    IOError(io::Error),
    NoMatchingImporter,
    NoTransactionError,

    Multi(Vec<BudgetError>),
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

impl From<serde_yaml::Error> for BudgetError {
    fn from(e: serde_yaml::Error) -> BudgetError {
        BudgetError::SerdeYamlError(e)
    }
}

impl From<io::Error> for BudgetError {
    fn from(e: io::Error) -> BudgetError {
        BudgetError::IOError(e)
    }
}

pub type BResult<T> = Result<T, BudgetError>;

// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    csv,
    std::{convert::From, io},
    toml,
};

#[derive(Debug)]
pub enum BudgetError {
    CSVError(csv::Error),
    NoCategoryFoundError(String),
    TomlDeError(toml::de::Error),
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

impl From<io::Error> for BudgetError {
    fn from(e: io::Error) -> BudgetError {
        BudgetError::IOError(e)
    }
}

pub type BResult<T> = Result<T, BudgetError>;

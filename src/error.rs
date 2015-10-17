use csv;
use std::convert::From;

#[derive(Debug)]
pub enum BudgetError {
    CSVError(csv::Error)
}

impl From<csv::Error> for BudgetError {
    fn from(e: csv::Error) -> BudgetError {
        BudgetError::CSVError(e)
    }
}


pub type BResult<T> = Result<T, BudgetError>;

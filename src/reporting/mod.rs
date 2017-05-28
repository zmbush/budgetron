use budgetronlib::fintime::Timeframe;
use loading::Transaction;
use reporting::by_account::ByAccount;
use reporting::by_timeframe::ByTimeframe;
use serde;
use std::borrow::Cow;
use std::fmt;

pub trait Reporter: Sized {
    type OutputType;

    fn report<'a, I>(&self, transactions: I) -> Self::OutputType
        where I: Iterator<Item = Cow<'a, Transaction>> + Clone;

    fn by_week(&self) -> ByTimeframe<Self> {
        ByTimeframe::new(self, Timeframe::Weeks(1))
    }

    fn by_month(&self) -> ByTimeframe<Self> {
        ByTimeframe::new(self, Timeframe::Months(1))
    }

    fn by_quarter(&self) -> ByTimeframe<Self> {
        ByTimeframe::new(self, Timeframe::Quarters(1))
    }

    fn for_account(&self, account: String) -> ByAccount<Self> {
        ByAccount::new(self, account)
    }
}

pub trait Report: fmt::Display + serde::Serialize {}

mod by_timeframe;
mod by_account;
mod net_worth;
mod database;
mod cashflow;
mod multi;

pub use reporting::cashflow::Cashflow;
pub use reporting::database::Database;
pub use reporting::net_worth::NetWorth;

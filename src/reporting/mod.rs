use budgetronlib::fintime::Timeframe;
use loading::Transaction;
use reporting::by_account::ByAccount;
use reporting::by_timeframe::ByTimeframe;
use serde;
use serde_json::Value;
use std::borrow::Cow;
use std::fmt;

pub trait Reporter: Sized {
    fn report<'a, I>(&self, transactions: I) -> Value
    where
        I: Iterator<Item = Cow<'a, Transaction>> + Clone;

    fn key(&self) -> Option<String>;

    fn by_week(&self) -> ByTimeframe<Self> {
        ByTimeframe::new(self, Timeframe::Weeks(1))
    }

    fn by_month(&self) -> ByTimeframe<Self> {
        ByTimeframe::new(self, Timeframe::Months(1))
    }

    fn by_quarter(&self) -> ByTimeframe<Self> {
        ByTimeframe::new(self, Timeframe::Quarters(1))
    }

    fn by_quarters(&self, quarters: i64) -> ByTimeframe<Self> {
        ByTimeframe::new(self, Timeframe::Quarters(quarters))
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
mod repeats;

pub use reporting::by_account::ByAccountReport;
pub use reporting::by_timeframe::ByTimeframeReport;
pub use reporting::cashflow::Cashflow;
pub use reporting::database::Database;
pub use reporting::net_worth::NetWorth;
pub use reporting::repeats::RepeatedTransactions;

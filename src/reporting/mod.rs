// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    crate::{
        loading::{Transaction, TransactionType},
        reporting::{by_account::ByAccount, by_timeframe::ByTimeframe},
    },
    budgetronlib::fintime::Timeframe,
    serde,
    serde_json::Value,
    std::{borrow::Cow, fmt},
};

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

    fn by_year(&self) -> ByTimeframe<Self> {
        ByTimeframe::new(self, Timeframe::Years(1))
    }

    fn for_account(&self, account: String) -> ByAccount<Self> {
        ByAccount::new(self, account)
    }

    fn excluding_tags(&self, tags: Vec<String>) -> ExcludingTags<Self> {
        ExcludingTags::new(self, tags)
    }

    fn only_tags(&self, tags: Vec<String>) -> OnlyTags<Self> {
        OnlyTags::new(self, tags)
    }

    fn only_type(&self, t: TransactionType) -> OnlyType<Self> {
        OnlyType::new(self, t)
    }

    fn only_owners(&self, t: Vec<String>) -> OnlyOwners<Self> {
        OnlyOwners::new(self, t)
    }
}

pub trait Report: fmt::Display + serde::Serialize {}

mod by_account;
mod by_timeframe;
mod cashflow;
mod categories;
mod config;
#[cfg(feature = "db")]
mod database;
mod excluding_tags;
mod income_expense_ratio;
mod list;
mod multi;
mod net_worth;
mod only_owners;
mod only_tags;
mod only_type;
mod rolling_budget;
mod timeseries;

#[cfg(feature = "db")]
pub use crate::reporting::database::Database;
pub use crate::reporting::{
    by_account::ByAccountReport, by_timeframe::ByTimeframeReport, cashflow::Cashflow,
    categories::Categories, config::ConfiguredReports, excluding_tags::ExcludingTags,
    income_expense_ratio::IncomeExpenseRatio, list::List, net_worth::NetWorth,
    only_owners::OnlyOwners, only_tags::OnlyTags, only_type::OnlyType,
    rolling_budget::RollingBudget,
};

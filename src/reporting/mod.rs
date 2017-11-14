// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use budgetronlib::fintime::Timeframe;
use loading::{Transaction, TransactionType};
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

    fn by_year(&self) -> ByTimeframe<Self> {
        ByTimeframe::new(self, Timeframe::Years(1))
    }

    fn for_account(&self, account: String) -> ByAccount<Self> {
        ByAccount::new(self, account)
    }

    fn excluding_tags(&self, tags: Vec<String>) -> ExcludingTags<Self> {
        ExcludingTags::new(self, tags)
    }

    fn only_type(&self, t: TransactionType) -> OnlyType<Self> {
        OnlyType::new(self, t)
    }
}

pub trait Report: fmt::Display + serde::Serialize {}

mod by_account;
mod by_timeframe;
mod cashflow;
mod categories;
mod config;
mod database;
mod excluding_tags;
mod list;
mod multi;
mod net_worth;
mod only_type;
mod rolling_budget;
mod timeseries;

pub use reporting::by_account::ByAccountReport;
pub use reporting::by_timeframe::ByTimeframeReport;
pub use reporting::cashflow::Cashflow;
pub use reporting::database::Database;
pub use reporting::net_worth::NetWorth;
pub use reporting::rolling_budget::{RollingBudget, RollingBudgetConfig};
pub use reporting::excluding_tags::ExcludingTags;
pub use reporting::config::ConfiguredReports;
pub use reporting::categories::Categories;
pub use reporting::only_type::OnlyType;
pub use reporting::list::List;

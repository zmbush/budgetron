// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {crate::loading::Transaction, budgetronlib::fintime::Timeframe, std::borrow::Cow};

mod data;
mod filters;
mod meta;

use self::meta::by_timeframe::ByTimeframe;

pub trait Reporter: Sized {
    fn report<'t>(
        &self,
        transactions: impl Iterator<Item = Cow<'t, Transaction>>,
    ) -> data::ConcreteReport;

    fn by_week(&self) -> ByTimeframe<Self> {
        ByTimeframe::new(&self, Timeframe::Weeks(1))
    }
}

mod cashflow;
mod categories;
mod config;
#[cfg(feature = "db")]
mod database;
mod income_expense_ratio;
mod list;
//mod multi;
mod net_worth;
mod rolling_budget;
mod timeseries;

#[cfg(feature = "db")]
pub use crate::reporting::database::Database;
pub use crate::reporting::{
    cashflow::Cashflow,
    categories::Categories,
    config::ConfiguredReports,
    income_expense_ratio::IncomeExpenseRatio,
    list::List,
    net_worth::NetWorth,
    rolling_budget::{RollingBudget, RollingBudgetConfig},
};

// Copyright 2019 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(target_arch = "wasm32")]
use {
    crate::{loading::Transaction, reporting::web::ConfiguredReportDataUi},
    std::collections::HashMap,
    yew::prelude::*,
};

pub enum ReportData {
    List(Vec<ReportData>),

    Report(ConcreteReport),
}

macro_rules! make_concrete_report {
    ($($variant:ident($type:ty)),*) => {
        #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
        #[serde(tag = "report_type")]
        pub enum ConcreteReport {
            $($variant($type)),*
        }

        $(
            impl From<$type> for ConcreteReport {
                fn from(r: $type) -> Self {
                    Self::$variant(r)
                }
            }
        )*

        #[cfg(target_arch = "wasm32")]
        impl ConcreteReport {
            pub fn view(
                &self,
                config: &crate::reporting::config::ReportConfig,
                transactions: &std::rc::Rc<HashMap<String, Transaction>>
            ) -> Html<ConfiguredReportDataUi> {
                match self {
                    $(ConcreteReport::$variant(inner) => inner.view(config, transactions)),*
                }
            }
        }
    };

    ($($variant:ident($type:ty)),*,) => {
        make_concrete_report!($($variant($type)),*);
    };
}

make_concrete_report! {
    NetWorth(super::net_worth::NetWorthReport),
    RollingBudet(super::rolling_budget::RollingBudgetReport),
    IncomeExpenseRatio(super::income_expense_ratio::IncomeExpenseReportType),
    Categories(super::categories::CategoriesReport),
    Cashflow(super::cashflow::CashflowReport),
}

impl<T> From<T> for ReportData
where
    T: Into<ConcreteReport>,
{
    fn from(r: T) -> Self {
        Self::Report(r.into())
    }
}

impl From<Vec<ReportData>> for ReportData {
    fn from(r: Vec<ReportData>) -> Self {
        Self::List(r)
    }
}

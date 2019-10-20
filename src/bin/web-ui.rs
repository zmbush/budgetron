// Copyright 2019 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![recursion_limit = "320"]

#[cfg(target_arch = "wasm32")]
mod ui {
    use {
        budgetron::{
            loading::Transaction,
            reporting::{
                web::{ConfiguredReportDataUi, DisplayMode, DisplayProps},
                ConfiguredReportData,
            },
        },
        failure::Error,
        std::{collections::HashMap, rc::Rc},
        yew::{
            format::{Json, Nothing},
            prelude::*,
            services::fetch::{FetchService, FetchTask, Request, Response},
        },
    };

    struct BudgetronWeb {
        link: ComponentLink<Self>,
        fetch: FetchService,
        display_props: DisplayProps,

        fetch_data: Option<FetchTask>,
        fetch_transactions: Option<FetchTask>,

        report_data: Option<Vec<Rc<ConfiguredReportData>>>,
        transactions: Option<Rc<HashMap<String, Transaction>>>,

        error: Option<String>,
    }

    enum Msg {
        FetchData,
        FetchTransactions,
        LoadFailed,
        FailedParse(Error),
        Data(Vec<ConfiguredReportData>),
        Transactions(HashMap<String, Transaction>),
        Toggle(DisplayMode),
    }

    impl Component for BudgetronWeb {
        type Message = Msg;
        type Properties = ();

        fn create(_: (), mut link: ComponentLink<Self>) -> Self {
            link.send_self(Msg::FetchData);
            link.send_self(Msg::FetchTransactions);

            Self {
                link,
                fetch: FetchService::new(),
                display_props: Default::default(),

                fetch_data: None,
                fetch_transactions: None,

                report_data: None,
                transactions: None,

                error: None,
            }
        }

        fn update(&mut self, msg: Msg) -> ShouldRender {
            match msg {
                Msg::FetchData => {
                    log::info!("Fetching data...");
                    self.fetch_data = Some(
                        self.fetch.fetch(
                            Request::get("http://localhost:5300/__/data.json")
                                .body(Nothing)
                                .expect("BOOT"),
                            self.link.send_back(
                                |resp: Response<Json<Result<Vec<ConfiguredReportData>, Error>>>| {
                                    let (meta, Json(body)) = resp.into_parts();
                                    if meta.status.is_success() {
                                        match body {
                                            Ok(report_data) => Msg::Data(report_data),
                                            Err(err) => Msg::FailedParse(err),
                                        }
                                    } else {
                                        log::error!("Failed to load data: {:?}", meta.status);
                                        Msg::LoadFailed
                                    }
                                },
                            ),
                        ),
                    );
                }
                Msg::FetchTransactions => {
                    log::info!("Fetching transactions...");
                    self.fetch_transactions = Some(
                        self.fetch.fetch(
                            Request::get("http://localhost:5300/__/transactions.json")
                                .body(Nothing)
                                .expect("Bod"),
                            self.link.send_back(
                                |resp: Response<
                                    Json<Result<HashMap<String, Transaction>, Error>>,
                                >| {
                                    let (meta, Json(body)) = resp.into_parts();
                                    if meta.status.is_success() {
                                        match body {
                                            Ok(transactions) => Msg::Transactions(transactions),
                                            Err(err) => Msg::FailedParse(err),
                                        }
                                    } else {
                                        log::error!(
                                            "Failed to load transactions: {:?}",
                                            meta.status
                                        );
                                        Msg::LoadFailed
                                    }
                                },
                            ),
                        ),
                    );
                }
                Msg::LoadFailed => {
                    self.error = Some("Could not load data".to_owned());
                }
                Msg::FailedParse(err) => {
                    self.error = Some(format!("Failed to parse the data: {}", err));
                }
                Msg::Data(report_data) => {
                    log::info!("Report data loaded!");
                    self.report_data = Some(
                        report_data
                            .into_iter()
                            .map(|report| Rc::new(report))
                            .collect(),
                    );
                }
                Msg::Transactions(transactions) => {
                    log::info!("Transaction dat loaded!");
                    self.transactions = Some(Rc::new(transactions));
                }
                Msg::Toggle(mode) => {
                    log::info!("Toggling {:?}", mode);
                    match mode {
                        DisplayMode::Simple => {} // Always show simple

                        DisplayMode::ByWeek => {
                            self.display_props.by_week = !self.display_props.by_week;
                        }
                        DisplayMode::ByMonth => {
                            self.display_props.by_month = !self.display_props.by_month;
                        }
                        DisplayMode::ByQuarter => {
                            self.display_props.by_quarter = !self.display_props.by_quarter;
                        }
                        DisplayMode::ByYear => {
                            self.display_props.by_year = !self.display_props.by_year;
                        }
                    }
                }
            }
            true
        }
    }

    impl BudgetronWeb {
        fn ui_for(
            &self,
            transactions: &Rc<HashMap<String, Transaction>>,
            data: &[Rc<ConfiguredReportData>],
            display_mode: DisplayMode,
        ) -> Html<Self> {
            if self.display_props.is_set(display_mode) || DisplayMode::Simple == display_mode {
                html! {{
                    for data.iter().map(|i| html! {
                        <ConfiguredReportDataUi
                            transactions={Rc::clone(transactions)}
                            data={Rc::clone(i)}
                            display={display_mode} />
                    })
                }}
            } else {
                html! {}
            }
        }

        fn timeframe_button(&self, display_mode: DisplayMode) -> Html<Self> {
            use DisplayMode::*;

            let mut classes = "waves-effect waves-light btn-small".to_owned();
            if !self.display_props.is_set(display_mode) {
                classes.push_str(" grey");
            }

            html! {
                <button
                    class={ classes }
                    onclick=|_| Msg::Toggle(display_mode)
                >
                    { display_mode }
                </button>
            }
        }
    }

    impl Renderable<BudgetronWeb> for BudgetronWeb {
        fn view(&self) -> Html<Self> {
            if let (Some(ref data), Some(ref transactions)) =
                (&self.report_data, &self.transactions)
            {
                html! {
                    <>
                        <div class="row">
                            <div class="col s12">
                                <div class="section">
                                    { self.timeframe_button(DisplayMode::ByWeek) }
                                    { self.timeframe_button(DisplayMode::ByMonth) }
                                    { self.timeframe_button(DisplayMode::ByQuarter) }
                                    { self.timeframe_button(DisplayMode::ByYear) }
                                </div>
                            </div>
                        </div>

                        { self.ui_for(transactions, data, DisplayMode::Simple) }
                        { self.ui_for(transactions, data, DisplayMode::ByWeek) }
                        { self.ui_for(transactions, data, DisplayMode::ByMonth) }
                        { self.ui_for(transactions, data, DisplayMode::ByQuarter) }
                        { self.ui_for(transactions, data, DisplayMode::ByYear) }
                    </>
                }
            } else if let Some(error) = &self.error {
                html! { <div>{ "Unable to load data: " }{ error }</div> }
            } else {
                html! {
                    <div>{ "Loading..." }</div>
                }
            }
        }
    }

    pub fn run() {
        web_logger::init();

        yew::start_app::<BudgetronWeb>()
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod ui {
    pub fn run() {
        println!("This does nothing outside of wasm");
    }
}

fn main() {
    ui::run();
}

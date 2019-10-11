// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(unused)]

use {
    budgetron::{
        loading,
        processing::{collate_all, Collator, ConfiguredProcessors},
        reporting::{ConfiguredReports, List},
    },
    budgetronlib::config,
    iron::prelude::*,
    mount::Mount,
    serde::Serialize,
    std::{borrow::Cow, path::Path},
    structopt::StructOpt,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "Budgetron", about = "Processes transactions into reports")]
struct Opt {
    #[structopt(short = "f", long = "file")]
    input_files: Vec<String>,

    #[structopt(short, long)]
    serve: bool,

    #[structopt(short, long, default_value = "3000")]
    port: u32,
}

#[cfg(feature = "db")]
use budgetron::reporting::Database;

fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    log::info!("Starting load");
    let transactions =
        loading::load_from_files(opt.input_files.into_iter()).expect("Unable to load files");

    let processors: ConfiguredProcessors =
        config::load_cfg("budgetronrc.toml").expect("Configured Processors failed to load");
    log::info!("Starting collation");
    let transactions =
        collate_all(transactions, &[Collator::Config(processors)]).expect("Unable to collate");
    let cow_transactions = transactions
        .iter()
        .map(|t| Cow::Borrowed(t))
        .collect::<Vec<_>>();

    let reports: ConfiguredReports =
        config::load_cfg("budgetronrc.toml").expect("Configured Reports failed to load");
    log::info!("Starting reports");
    let report = reports.report(cow_transactions.into_iter());

    println!("{:?}", serde_json::to_string(&report));

    let cow_transactions = transactions
        .iter()
        .map(|t| Cow::Borrowed(t))
        .collect::<Vec<_>>();
    #[cfg(feature = "db")]
    let transaction_list = (List, Database).report(cow_transactions.into_iter());

    #[cfg(not(feature = "db"))]
    let transaction_list = List.report(cow_transactions.into_iter());

    if opt.serve {
        let mut mount = Mount::new();
        mount.mount("/", staticfile::Static::new(Path::new("web/static")));
        mount.mount("__/data.json", JsonHandler { data: report });
        mount.mount(
            "__/transactions.json",
            JsonHandler {
                data: transaction_list,
            },
        );
        Iron::new(mount)
            .http(format!("0.0.0.0:{}", opt.port))
            .unwrap();
    }
}

struct JsonHandler<T: Serialize> {
    data: T,
}

impl<T: Serialize + Send + Sync + 'static> iron::middleware::Handler for JsonHandler<T> {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((
            iron::status::Ok,
            serde_json::to_string(&self.data).unwrap(),
        )))
    }
}

// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(unused)]

use budgetron::loading;
use budgetron::processing::{collate_all, Collator, ConfiguredProcessors};
use budgetron::reporting::{ConfiguredReports, Database, List, Reporter};
use budgetronlib::config;
use clap::{App, Arg};
use iron::prelude::*;
use mount::Mount;
use serde::Serialize;
use std::borrow::Cow;
use std::path::Path;

fn main() {
    env_logger::init().expect("Unable to set up env_logger");

    let matches = App::new("Collator")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Zachary Bush <zach@zmbush.com>")
        .about("Parse exports and convert to standard format")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Export from a supported institution")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("serve")
                .long("serve")
                .help("Start server to view reports"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .value_name("PORT")
                .long("port")
                .help("Port to host the server at")
                .default_value("3000")
                .takes_value(true),
        )
        .get_matches();

    let transactions = if let Some(files) = matches.values_of("file") {
        loading::load_from_files(files).expect("Unable to load files")
    } else {
        Vec::new()
    };

    let processors: ConfiguredProcessors =
        config::load_cfg("budgetronrc.toml").expect("Configured Processors failed to load");
    let transactions =
        collate_all(transactions, &[Collator::Config(processors)]).expect("Unable to collate");
    let cow_transactions = transactions
        .iter()
        .map(|t| Cow::Borrowed(t))
        .collect::<Vec<_>>();

    let reports: ConfiguredReports =
        config::load_cfg("budgetronrc.toml").expect("Configured Reports failed to load");
    let report = reports.report(cow_transactions.into_iter());

    let cow_transactions = transactions
        .iter()
        .map(|t| Cow::Borrowed(t))
        .collect::<Vec<_>>();
    let transaction_list = (List, Database).report(cow_transactions.into_iter());

    if matches.is_present("serve") {
        let mut mount = Mount::new();
        mount.mount("/", staticfile::Static::new(Path::new("web/static")));
        mount.mount("__/data.json", JsonHandler { data: report });
        mount.mount(
            "__/transactions.json",
            JsonHandler {
                data: transaction_list,
            },
        );
        let port = matches.value_of("port").unwrap();
        Iron::new(mount).http(format!("0.0.0.0:{}", port)).unwrap();
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

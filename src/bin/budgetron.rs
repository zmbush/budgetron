#![deny(unused_extern_crates, unused)]

extern crate budgetron;
extern crate budgetronlib;
extern crate clap;
extern crate env_logger;
extern crate serde;
extern crate serde_json;

extern crate iron;
extern crate mount;
extern crate staticfile;

use budgetron::loading;
use budgetron::processing::{collate_all, Collator, ConfiguredProcessors, TransferCollator};
use budgetron::reporting::{ConfiguredReports, Database, Reporter};
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
            Arg::with_name("transfers")
                .short("t")
                .long("transfers")
                .value_name("HORIZON")
                .help("The number of transactions to look through to find transfer.")
                .default_value("100")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("serve")
                .long("serve")
                .help("Start server to view reports"),
        )
        .get_matches();

    let transactions = if let Some(files) = matches.values_of("file") {
        loading::load_from_files(files).expect("Unable to load files")
    } else {
        Vec::new()
    };

    let mut collations = Vec::new();
    if let Some(Ok(horizon)) = matches.value_of("transfers").map(|t| t.parse()) {
        collations.push(Collator::Transfers(TransferCollator::new(horizon)));
    }
    let processors: ConfiguredProcessors =
        config::load_cfg("budgetronrc.toml").expect("Configured Processors failed to load");
    collations.push(Collator::Config(processors));

    let transactions = collate_all(transactions, collations).expect("Unable to collate");
    let cow_transactions = transactions
        .iter()
        .map(|t| Cow::Borrowed(t))
        .collect::<Vec<_>>();

    let reports: ConfiguredReports =
        config::load_cfg("budgetronrc.toml").expect("Configured Reports failed to load");
    let report = (Database, reports).report(cow_transactions.into_iter());

    if matches.is_present("serve") {
        let mut mount = Mount::new();
        mount.mount("/", staticfile::Static::new(Path::new("web/static")));
        mount.mount("__/data.json", JsonHandler { data: report });
        Iron::new(mount).http("0.0.0.0:3000").unwrap();
    }
}

struct JsonHandler<T: Serialize> {
    data: T,
}

impl<T: Serialize + Send + Sync + 'static> iron::middleware::Handler for JsonHandler<T> {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((
            iron::status::Ok,
            serde_json::to_string_pretty(&self.data).unwrap(),
        )))
    }
}

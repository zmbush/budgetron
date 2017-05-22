#[deny(unused_extern_crates)]

extern crate budgetronlib;
extern crate env_logger;
extern crate clap;
extern crate csv;
extern crate budgetron;

use budgetron::loading;
use budgetron::processing::{collate_all, TransferCollator, Collator};
use budgetronlib::config::{self, CategoryConfig};
use clap::{App, Arg};
use csv::Writer;
use std::io;

fn main() {
    env_logger::init().expect("Unable to set up env_logger");

    let matches = App::new("Collator")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Zachary Bush <zach@zmbush.com>")
        .about("Parse exports and convert to standard format")
        .arg(Arg::with_name("file")
                 .short("f")
                 .long("file")
                 .value_name("FILE")
                 .help("Export from a supported institution")
                 .takes_value(true)
                 .multiple(true))
        .arg(Arg::with_name("transfers")
                 .short("t")
                 .long("transfers")
                 .value_name("HORIZON")
                 .help("The number of transactions to look through to find transfer.")
                 .takes_value(true))
        .get_matches();

    let category_config: CategoryConfig = config::load_cfg("budgetronrc.toml")
        .expect("Unable to load budgetronrc.toml");

    let transactions = if let Some(files) = matches.values_of("file") {
        loading::load_from_files(files, &category_config).expect("Unable to load files")
    } else {
        Vec::new()
    };

    let mut collations: Vec<Box<Collator>> = Vec::new();
    if let Some(Ok(horizon)) = matches.value_of("transfers").map(|t| t.parse()) {
        collations.push(Box::new(TransferCollator::new(horizon)));
    }

    let transactions = collate_all(transactions, collations).expect("Unable to collate");

    let mut writer = Writer::from_writer(io::stdout());
    for transaction in transactions {
        writer
            .serialize(transaction)
            .expect("Could not write transaction!");
    }
}

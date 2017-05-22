#[deny(unused_extern_crates)]

extern crate budgetronlib;
extern crate env_logger;
extern crate clap;
extern crate csv;
extern crate budgetron;

use budgetron::loading;
use budgetronlib::config::{self, CategoryConfig};
use clap::{App, Arg};
use csv::Writer;
use std::io;

fn main() {
    env_logger::init().expect("Unable to set up env_logger");

    let matches = App::new("Transactor")
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
        .get_matches();

    let category_config: CategoryConfig = config::load_cfg("budgetronrc.toml")
        .expect("Unable to load budgetronrc.toml");


    let transactions = if let Some(files) = matches.values_of("file") {
        loading::load_from_files(files, &category_config).expect("Unable to load files")
    } else {
        Vec::new()
    };

    let mut writer = Writer::from_writer(io::stdout());
    for transaction in transactions {
        writer
            .serialize(transaction)
            .expect("Could not write transaction!");
    }
}

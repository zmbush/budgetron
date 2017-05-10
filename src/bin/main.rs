#[macro_use]
extern crate log;
extern crate transactor;
extern crate budgetronlib;
extern crate env_logger;
extern crate clap;

use budgetronlib::config::{self, CategoryConfig};
use clap::{App, Arg};

fn main() {
    env_logger::init().expect("Unable to set up env_logger");

    let matches = App::new("Budgetron")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Zachary Bush <zach@zmbush.com>")
        .about("Makes you a budget for great success")
        .arg(Arg::with_name("logix-file")
                 .short("l")
                 .long("logix-file")
                 .value_name("FILE")
                 .help("File from Logix")
                 .takes_value(true)
                 .multiple(true))
        .arg(Arg::with_name("mint-file")
                 .short("m")
                 .long("mint-file")
                 .value_name("FILE")
                 .help("File from mint")
                 .takes_value(true)
                 .multiple(true))
        .arg(Arg::with_name("output-dir")
                 .short("o")
                 .long("output-dir")
                 .value_name("DIR")
                 .help("Directory for output reports")
                 .takes_value(true))
        .arg(Arg::with_name("send-email").short("e").long("send-email"))
        .get_matches();

    let category_config: CategoryConfig = config::load_cfg("budgetronrc.toml")
        .expect("Unable to load budgetronrc.toml");

    if let Some(mint_files) = matches.values_of("mint-file") {
        for file in mint_files {
            info!("Opening mint file: {}", file);
            let export = transactor::from_file::<transactor::mint::MintExport>(&file,
                                                                               &category_config)
                    .expect("Unable to load files");
            println!("Export: {:?}", export);
        }
    }

    if let Some(logix_files) = matches.values_of("logix-file") {
        for file in logix_files {
            info!("Opening logix file: {}", file);
            let export = transactor::from_file::<transactor::logix::LogixExport>(&file,
                                                                                 &category_config)
                    .expect("Unable to load file");
            println!("Export: {:?}", export);
        }
    }
}

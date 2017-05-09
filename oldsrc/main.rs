#![deny(unused)]

extern crate chrono;
extern crate clap;
extern crate csv;
extern crate data_store;
extern crate email;
extern crate env_logger;
extern crate handlebars;
extern crate lettre;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rustc_serialize;
extern crate toml;

mod budget;
mod categories;
mod common;
mod error;
mod exports;
mod fintime;
mod config;
mod mailer;

use budget::Budget;
use clap::{App, Arg};
use common::Transactions;
use error::BResult;
use exports::{LogixExport, MintExport};
use fintime::Timeframe;
use fintime::Timeframe::*;
use std::{fs, io};
use std::path::Path;

fn generate_budget(d: &Path,
                   period: &Timeframe,
                   periods: usize,
                   transactions: &Transactions)
                   -> BResult<bool> {
    let budget = try! {
            Budget::calculate(period, periods, transactions)
        };
    let filename = format!("Budget for {} ending on {:#}.csv",
                           if periods == 0 {
                               format!("last {}", period)
                           } else if periods == 1 {
        format!("1 {} period", period)
    } else {
        format!("{} {} periods", periods, period)
    },
                           budget.end_date);
    try!(budget.write_to_file(d.join(filename)));

    Ok(true)
}

fn main() {
    env_logger::init().unwrap();

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
        .arg(Arg::with_name("send-email")
                 .short("e")
                 .long("send-email"))
        .get_matches();

    let cfg: config::SecureConfig =
        config::load_cfg(".budgetron.toml").expect("Couldn't load email config");

    let category_cfg: config::CategoryConfig =
        config::load_cfg("budgetronrc.toml").expect("Unable to load budgetronrc.toml");

    let mut transactions = Transactions::new(&category_cfg);

    if let Some(logix_files) = matches.values_of("logix-file") {
        for file in logix_files {
            println!("Opening logix file: {}", file);
            transactions
                .load_records::<LogixExport>(&file)
                .expect(&format!("Couldn't load logix transactions from {}", file));
        }
    }

    if let Some(mint_files) = matches.values_of("mint-file") {
        for file in mint_files {
            println!("Opening mint file: {}", file);
            transactions
                .load_records::<MintExport>(&file)
                .expect(&format!("Couldn't load mint transactions from {}", file));
        }
    }

    transactions.collate();

    let transaction_database = data_store::Transactions::new_from_env();
    transactions.write_to_db(&transaction_database);

    if let Some(output_dir) = matches.value_of("output-dir") {
        let d = Path::new(&output_dir);

        let metadata = match fs::metadata(d) {
            Ok(m) => m,
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    if let Err(e) = fs::create_dir_all(d) {
                        println!("Unable to create directory '{}' ({})", d.display(), e);
                        return;
                    }
                    fs::metadata(&d).expect("Creation of directory failed")
                } else {
                    println!("Unable to create directory {}", e);
                    return;
                }
            },
        };

        if !metadata.is_dir() {
            println!("{} exists and is not a directory", d.display());
        }

        let mut out = csv::Writer::from_file(d.join("All Transactions.csv")).unwrap();
        out.write(["date",
                    "person",
                    "description",
                    "original description",
                    "amount",
                    "type",
                    "category",
                    "original category",
                    "account",
                    "labels",
                    "notes"]
                           .iter())
            .unwrap();
        for transaction in transactions.iter() {
            out.encode(transaction).unwrap();
        }

        let _ = generate_budget(d, &Months(1), 4, &transactions);
        let _ = generate_budget(d, &Months(1), 12, &transactions);
        let _ = generate_budget(d, &Weeks(2), 6, &transactions);
        let _ = generate_budget(d, &Quarters(1), 4, &transactions);
        let _ = generate_budget(d, &Years(1), 4, &transactions);

    }
    if matches.is_present("send-email") {
        if let Some(email_cfg) = cfg.email {
            let budget = Budget::calculate(&Months(1), 2, &transactions).unwrap();
            mailer::email_budget(&email_cfg, &budget);
        } else {
            println!("Can't use --send-email without email config in ~/.budgetron.toml");
        }
    }
}

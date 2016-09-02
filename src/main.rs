#![feature(
    plugin,
    custom_attribute,
    custom_derive
)]
#![plugin(phf_macros)]
#![plugin(tojson_macros)]
#![deny(unused)]

extern crate csv;
extern crate docopt;
extern crate phf;
extern crate rustc_serialize;
extern crate chrono;
extern crate lettre;
extern crate env_logger;
extern crate handlebars;
extern crate toml;
extern crate email;
extern crate data_store;

mod budget;
mod categories;
mod common;
mod error;
mod exports;
mod fintime;
mod config;
mod mailer;

use common::Transactions;
use fintime::Timeframe;
use fintime::Timeframe::*;
use exports::{LogixExport, MintExport};
use docopt::Docopt;
use std::{fs, io};
use std::path::Path;
use budget::Budget;
use error::BResult;

#[allow(unused)]
#[rustfmt_skip]
const USAGE: &'static str = "
Parse export csvs from Molly and Zach's tools

Usage:
    budgetron [--logix-file=<file> ...] [--mint-file=<file> ...] --output-dir=<directory> [options]
    budgetron (-h | --help)

    Options:
    -h --help           Show this screen.
    --logix-file=<file>
    --mint-file=<file>
    --output-dir=<directory>
    --send-email
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_logix_file: Vec<String>,
    flag_mint_file: Vec<String>,
    flag_output_dir: String,
    flag_send_email: bool,
}

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

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let cfg: config::SecureConfig = config::load_cfg(".budgetron.toml")
        .expect("Couldn't load email config");

    let category_cfg: config::CategoryConfig = config::load_cfg("budgetronrc.toml")
        .expect("Unable to load budgetronrc.toml");

    let mut transactions = Transactions::new(&category_cfg);

    for file in args.flag_logix_file {
        transactions.load_records::<LogixExport>(&file)
            .expect(&format!("Couldn't load logix transactions from {}", file));
    }

    for file in args.flag_mint_file {
        transactions.load_records::<MintExport>(&file)
            .expect(&format!("Couldn't load mint transactions from {}", file));
    }

    transactions.collate();

    let trans_db = data_store::Transactions::new_from_env();
    let mut all_transactions = Vec::new();
    for t in transactions.iter() {
        all_transactions.push(data_store::models::NewTransaction {
            date: t.date.date.naive_utc(),
            person: match t.person {
                common::Person::Molly => "Molly",
                common::Person::Zach => "Zach",
            },
            description: &t.description,
            original_description: &t.original_description,
            amount: t.amount,
            transaction_type: match t.transaction_type {
                common::TransactionType::Debit => "Debit",
                common::TransactionType::Credit => "Credit",
            },
            category: &t.category,
            original_category: &t.original_category,
            account_name: &t.account_name,
            labels: &t.labels,
            notes: &t.notes,
        });
    }
    trans_db.set_transactions(&all_transactions);

    let d = Path::new(&args.flag_output_dir);

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

    if args.flag_send_email {
        if let Some(email_cfg) = cfg.email {
            let budget = Budget::calculate(&Months(1), 2, &transactions).unwrap();
            mailer::email_budget(&email_cfg, &budget);
        } else {
            println!("Can't use --send-email without email config in ~/.budgetron.toml");
        }
    }
}

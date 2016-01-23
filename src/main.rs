#![feature(plugin, op_assign_traits, augmented_assignments)]
#![plugin(phf_macros)]
#![deny(unused)]

extern crate csv;
extern crate docopt;
extern crate phf;
extern crate rustc_serialize;
extern crate chrono;

mod budget;
mod categories;
mod common;
mod error;
mod exports;
mod fintime;

use common::Transactions;
use fintime::Timeframe;
use fintime::Timeframe::*;
use exports::{MintExport, LogixExport};
use rustc_serialize::Decodable;
use docopt::Docopt;
use std::{fs, io};
use std::path::Path;
use budget::Budget;
use error::BResult;

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
    --week-starts-on=<weekday>  Day that week starts on (e.g. Monday) [Default: Monday]
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_logix_file: Vec<String>,
    flag_mint_file: Vec<String>,
    flag_output_dir: String,
    flag_week_starts_on: String,
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
    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());

    let mut transactions = Transactions::new();

    for file in args.flag_logix_file {
        transactions.load_records::<LogixExport>(&file)
                    .expect(&format!("Couldn't load logix transactions from {}", file));
    }

    for file in args.flag_mint_file {
        transactions.load_records::<MintExport>(&file)
                    .expect(&format!("Couldn't load mint transactions from {}", file));
    }

    transactions.collate();

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
        }
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

    let _ = generate_budget(d, &Months(1), 3, &transactions);
    let _ = generate_budget(d, &Weeks(2), 6, &transactions);
}

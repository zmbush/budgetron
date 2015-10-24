#![feature(plugin, fmt_flags)]
#![plugin(phf_macros)]
// #![deny(unused)]

extern crate csv;
extern crate docopt;
extern crate phf;
extern crate rustc_serialize;
extern crate time;

mod categories;
mod common;
mod error;
mod exports;

use common::{Transactions, TransactionType, Date};
use exports::{MintExport, LogixExport};
use rustc_serialize::Decodable;
use docopt::Docopt;
use std::{fs, io};
use std::path::Path;
use std::collections::HashMap;

const USAGE: &'static str = "
Parse export csvs from Molly and Zach's tools

Usage:
    budgetron [--logix-file=<file> ...] [--mint-file=<file> ...] --output-dir=<directory>
    budgetron (-h | --help)

Options:
    -h --help           Show this screen.
    --logix-file=<file>
    --mint-file=<file>
    --output-dir=<directory>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_logix_file: Vec<String>,
    flag_mint_file: Vec<String>,
    flag_output_dir: String
}

fn write_pivot_table(d: &Path, time_frame: &str, transactions: &Transactions) {
    let start = Date::ago(time_frame);
    let end = Date::today();
    let mut amounts = HashMap::new();
    for t in transactions.iter() {
        if t.transaction_type == TransactionType::Debit &&
                t.date >= start && t.date < end {
            *amounts.entry(&t.category).or_insert(0.0) += t.amount;
        }
    }
    let mut out = csv::Writer::from_file(
        d.join(format!("by_categories_{}_{:#}_{:#}.csv", time_frame, start, end))).unwrap();
    out.write(["category", "amount"].iter());
    for key in amounts.keys() {
        out.write([key.clone(), &amounts[key].to_string()].iter());
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    println!("{:?}", args.flag_logix_file);

    let mut transactions = Transactions::new();

    for file in args.flag_logix_file {
        transactions.load_records::<LogixExport>(&file)
            .expect(&format!(
                    "Couldn't load logix transactions from {}",
                    file));
    }

    for file in args.flag_mint_file {
        transactions.load_records::<MintExport>(&file)
            .expect(&format!(
                    "Couldn't load mint transactions from {}",
                    file));
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

    let mut out = csv::Writer::from_file(d.join("out.csv")).unwrap();
    out.write(["date", "person", "description", "original description",
                "amount", "type", "category", "original category",
                "account", "labels", "notes"].iter()).unwrap();
    for transaction in transactions.iter() {
        out.encode(transaction).unwrap();
    }

    write_pivot_table(d, "1w", &transactions);
    write_pivot_table(d, "1m", &transactions);
    write_pivot_table(d, "6m", &transactions);
    write_pivot_table(d, "1q", &transactions);
    write_pivot_table(d, "2q", &transactions);
}

#![feature(plugin)]
#![plugin(phf_macros)]
#![deny(unused)]

extern crate phf;
extern crate csv;
extern crate rustc_serialize;
extern crate docopt;

mod common;
mod exports;
mod error;
mod categories;

use common::Transactions;
use exports::{MintExport, LogixExport};
use rustc_serialize::Decodable;
use docopt::Docopt;

const USAGE: &'static str = "
Parse export csvs from Barry and Zach's tools

Usage:
    budgetron [--logix-file=<file>] [--mint-file=<file>]
    budgetron (-h | --help)

Options:
    -h --help           Show this screen.
    --logix-file=<file>
    --mint-file=<file>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_logix_file: Vec<String>,
    flag_mint_file: Vec<String>
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let mut transactions = Transactions::new();

    for file in args.flag_logix_file {
        transactions.load_records::<LogixExport>(&file).unwrap();
    }

    for file in args.flag_mint_file {
        transactions.load_records::<MintExport>(&file).unwrap();
    }

    transactions.sort();

    let mut out = csv::Writer::from_file("data/out.csv").unwrap();
    out.write(["date", "person", "description", "original description",
                "amount", "type", "category", "original category",
                "account", "labels", "notes"].iter()).unwrap();
    for transaction in transactions.iter() {
        out.encode(transaction).unwrap();
    }
}

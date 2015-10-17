#![deny(unused)]

extern crate csv;
extern crate rustc_serialize;
extern crate docopt;

mod common;
mod exports;
mod error;

use common::{TransactionType, Genericize, Transaction};
use exports::{MintExport, LogixExport};
use rustc_serialize::Decodable;
use error::BResult;
use docopt::Docopt;

const USAGE: &'static str = "
Parse export csvs from Molly and Zach's tools

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

fn load_records<ExportType>(filename: &str, transactions: &mut Vec<Transaction>) -> BResult<i32>
        where ExportType: Genericize + Decodable {
    let mut rdr = try!(csv::Reader::from_file(filename));
    let mut count = 0;
    for record in rdr.decode() {
        let record: ExportType = try!(record);
        transactions.push(record.genericize());
        count += 1;
    }
    Ok(count)
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let mut transactions = Vec::new();

    for file in args.flag_logix_file {
        load_records::<LogixExport>(&file, &mut transactions).unwrap();
    }

    for file in args.flag_mint_file {
        load_records::<MintExport>(&file, &mut transactions).unwrap();
    }

    transactions.sort_by(|a, b| a.date.cmp(&b.date));

    let mut income = 0.0;
    let mut expense = 0.0;
    for transaction in transactions {
        if transaction.date.year == 2015 && transaction.date.month == 9 {
            match transaction.transaction_type {
                TransactionType::Debit => expense += transaction.amount,
                TransactionType::Credit => income += transaction.amount
            }
            println!("{:?} {} ({:?} / {}) {}",
                     transaction.person,
                     transaction.amount,
                     transaction.transaction_type,
                     transaction.category,
                     transaction.description);
        }
    }

    println!("");
    println!("Income: {}", income);
    println!("Expense: {}", expense);
}

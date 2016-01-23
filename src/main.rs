#![feature(plugin, op_assign_traits, augmented_assignments, convert)]
#![plugin(phf_macros)]
// #![deny(unused)]

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

use common::{Transactions, TransactionType};
use fintime::{Date, Timeframe};
use fintime::Timeframe::*;
use exports::{MintExport, LogixExport};
use rustc_serialize::Decodable;
use docopt::Docopt;
use std::{fs, io};
use std::path::Path;
use std::collections::{HashMap, HashSet};
use budget::Budget;
use error::BResult;

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
    flag_week_starts_on: String
}

fn write_aligned_pivot_table(d: &Path, duration: &Timeframe,
                             end_ago: &Timeframe,
                             transactions: &Transactions) {
    let now = transactions.transactions.last().unwrap().date;
    let end_ago_approx = now - end_ago;
    let mut start_date = end_ago_approx - duration;
    match *duration {
        Weeks(_) => start_date.align_to_week(),
        Months(_) => start_date.align_to_month(),
        Quarters(_) => start_date.align_to_quarter(),
        Years(_) => start_date.align_to_year(),
        Days(_) => {}
    }
    let end_date = start_date + duration;

    let mut amounts = HashMap::new();
    for t in transactions.iter() {
        if t.transaction_type == TransactionType::Debit &&
                t.date >= start_date && t.date < end_date {
            *amounts.entry(&t.category).or_insert(0.0) += t.amount;
        }
    }
    let mut out = csv::Writer::from_file(
        d.join(format!("by_categories_{:#}_{:#}_{:#}_{:#}.csv", duration, end_ago, start_date, end_date))).unwrap();
    out.write(["category", "amount"].iter());
    for key in amounts.keys() {
        out.write([key.clone(), &amounts[key].to_string()].iter());
    }
}

fn write_pivot_table(d: &Path, time_frame: &Timeframe,
                     transactions: &Transactions) {
    write_aligned_pivot_table(d, time_frame, &Days(0), transactions);
}

fn cell(col: usize, row: usize) -> String {
    format!("{}{}", ('A' as usize + col) as u8 as char, row)
}

fn generate_budget(d: &Path, period: &Timeframe, periods: usize,
                   transactions: &Transactions) -> BResult<bool> {
    let budget = try! {
        Budget::calculate(period, periods, transactions)
    };
    try!(budget.write_to_file(d.join(format!("budget_{:#}_for_{:#}.csv", period, periods))));

    Ok(true)
}

fn print_tpm_report(tt: TransactionType, categories: Vec<&str>, transactions: &Transactions) {
    let mut months = HashMap::new();
    for t in transactions.iter() {
        if t.transaction_type == tt {
            for c in &categories {
                if &t.category == c {
                    *months.entry((t.date.year(), t.date.month())).or_insert(0.0) += t.amount;
                }
            }
        }
    }
    let ms = {
        let mut tmp: Vec<(_, _)> = months.keys().cloned().collect();
        tmp.sort();
        tmp
    };

    for (year, month) in ms {
        println!("{}/{}: ${:.2}", month, year, months[&(year, month)]);
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    println!("{:#?}", args);

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

    write_pivot_table(d, &Weeks(1), &transactions);
    write_pivot_table(d, &Months(1), &transactions);
    write_pivot_table(d, &Months(6), &transactions);
    write_pivot_table(d, &Quarters(1), &transactions);
    write_pivot_table(d, &Quarters(2), &transactions);


    write_aligned_pivot_table(d, &Months(1), &Months(2), &transactions);
    write_aligned_pivot_table(d, &Months(1), &Months(1), &transactions);

    generate_budget(d, &Months(1), 3, &transactions);
    generate_budget(d, &Weeks(2), 6, &transactions);



    // print_tpm_report(TransactionType::Credit, vec!["Income"], &transactions);
    print_tpm_report(TransactionType::Debit, vec!["Bills", "Insurance"], &transactions);
    //print_tpm_report(TransactionType::Debit, vec!["Groceries"], &transactions);
}

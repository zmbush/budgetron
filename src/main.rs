#![feature(plugin, op_assign_traits, augmented_assignments, convert)]
#![plugin(phf_macros)]
// #![deny(unused)]

extern crate csv;
extern crate docopt;
extern crate phf;
extern crate rustc_serialize;
extern crate chrono;

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

#[derive(Debug, Clone)]
struct BudgetPeriodAmount {
    start: Date,
    end: Date,
    amount: f64
}

#[derive(Debug)]
struct BudgetCategory {
    name: String,
    previous_periods: Vec<f64>,
    current_period: f64,
    goal: f64
}

#[derive(Debug)]
struct Budget {
    period_start_dates: Vec<Date>,
    current_period_start_date: Date,
    end_date: Date,
    categories: HashMap<String, BudgetCategory>,
    has_historical: bool
}

fn generate_budget(period: &Timeframe, periods: usize, transactions: &Transactions) {
    // let mut data = HashMap::new();

    let now = transactions.transactions.last().unwrap().date;
    let mut start_date = now;
    for _ in 0..periods {
        start_date -= period;
    }
    match *period {
        Weeks(_) => start_date.align_to_week(),
        Months(_) => start_date.align_to_month(),
        Quarters(_) => start_date.align_to_quarter(),
        Years(_) => start_date.align_to_year(),
        Days(_) => {}
    }

    let mut end_date = start_date + period;
    let mut sd = start_date;
    let mut ed = end_date;
    let mut ix = 0;

    let mut budget = Budget {
        period_start_dates: vec![start_date],
        current_period_start_date: now,
        end_date: now,
        categories: HashMap::new(),
        has_historical: periods > 0
    };

    let factor = period / Months(1);
    for limited_category in categories::LIMITS.keys().cloned() {
        budget.categories.insert(limited_category.to_owned(), BudgetCategory {
            name: limited_category.to_owned(),
            previous_periods: vec![0.0; periods],
            current_period: 0.0,
            goal: categories::LIMITS[limited_category] * factor
        });
    }

    for t in transactions.iter() {
        if t.date >= end_date {
            ix += 1;
            start_date += period;
            end_date += period;

            if ix < periods {
                budget.period_start_dates.push(start_date);
            } else if ix == periods {
                budget.current_period_start_date = start_date;
            }
        }

        if t.transaction_type == TransactionType::Debit &&
                t.date >= start_date && t.date < end_date {
            let ref mut budget_category = budget.categories.entry(t.category.to_owned())
                .or_insert(BudgetCategory {
                    name: t.category.to_owned(),
                    previous_periods: vec![0.0; periods],
                    current_period: 0.0,
                    goal: 0.0
                });

            if ix < periods {
                *budget_category.previous_periods.get_mut(ix)
                    .expect(&format!("Tried to get index {}. Too big {}", ix, periods)) += t.amount;
            } else if ix == periods {
                budget_category.current_period += t.amount;
            }
        }
    }

    println!("Bud: {:#?}", budget);

    let mut keys: Vec<_> = budget.categories.keys().collect();
    let mut outfile = csv::Writer::from_file("budget.csv").unwrap();
    keys.sort();

    let mut current_row = Vec::new();
    current_row.push("Category".to_owned());
    if budget.has_historical {
        for period_start in &budget.period_start_dates {
            current_row.push(format!("{} - {}", period_start, period_start + period - Days(1)));
        }
        current_row.push("Average".to_owned());
    }
    current_row.push(format!("{} - {}", budget.current_period_start_date, budget.end_date));
    current_row.append(&mut vec!["Target Budget".to_owned(), "Budget Left".to_owned()]);
    outfile.write(current_row.iter());
    for (row, category_name) in keys.iter().enumerate() {
        current_row.clear();

        let ref category = budget.categories[*category_name];
        current_row.push(format!("{}", category_name));

        if budget.has_historical {
            for value in &category.previous_periods {
                current_row.push(format!("${:.2}", value));
            }
            current_row.push(format!("=AVERAGE({}:{})",
                                     cell(1, row + 2),
                                     cell(category.previous_periods.len(), row + 2)));
        }

        current_row.push(format!("${:.2}", category.current_period));
        current_row.push(format!("${:.2}", category.goal));
        current_row.push(format!("= {} - {}",
                                 cell(if budget.has_historical { periods + 3 } else { 2 }, row + 2),
                                 cell(if budget.has_historical { periods + 2 } else { 1 }, row + 2)));

        outfile.write(current_row.iter());
    }

    current_row.clear();
    current_row.push("Total".to_owned());
    for i in 0..if budget.has_historical { periods + 4 } else { 3 } {
        current_row.push(format!("=sum({}:{})", cell(i + 1, 2), cell(i + 1, keys.len() + 1)));
    }
    outfile.write(current_row.iter());
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

    generate_budget(&Months(1), 3, &transactions);



    // print_tpm_report(TransactionType::Credit, vec!["Income"], &transactions);
    print_tpm_report(TransactionType::Debit, vec!["Bills", "Insurance"], &transactions);
    //print_tpm_report(TransactionType::Debit, vec!["Groceries"], &transactions);
}

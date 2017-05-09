use common::{TransactionType, Transactions};
use csv;
use error::{BResult, BudgetError};
use fintime::{Date, Timeframe};
use fintime::Timeframe::*;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
// use serde_json::value::{ToJson, Value};
// use rustc_serialize::json::{Json, ToJson};

fn cell(col: usize, row: usize) -> String {
    format!("{}{}", ('A' as usize + col) as u8 as char, row)
}

#[derive(Debug, Serialize)]
struct BudgetCategory {
    name: String,
    previous_periods: Vec<f64>,
    current_period: f64,
    goal: f64,
}

/*impl ToJson for BudgetCategory {
    fn to_json(&self) -> Value {
        let mut m = HashMap::new();
        m.insert("name".to_owned(), self.name.to_json());
        m.insert("previous_periods".to_owned(),
                 self.previous_periods.to_json());
        m.insert("current_period".to_owned(), self.current_period.to_json());
        m.insert("most_recent_period".to_owned(),
                 self.previous_periods[self.previous_periods.len() - 1].to_json());
        m.insert("goal".to_owned(), self.goal.to_json());
        m.insert("remains".to_owned(),
                 (self.goal - self.current_period).to_json());
        m.insert("over_budget".to_owned(),
                 (self.current_period > self.goal).to_json());

        m.to_json()
    }
}*/

impl BudgetCategory {
    pub fn write_to_file<W: Write>(&self,
                                   row: usize,
                                   writer: &mut csv::Writer<W>)
                                   -> BResult<usize> {
        let mut current_row = Vec::new();

        current_row.push(format!("{}", self.name));

        let base_period = if self.previous_periods.len() > 0 {
            self.previous_periods.len() + 1
        } else {
            0
        };

        if self.previous_periods.len() > 0 {
            for value in &self.previous_periods {
                current_row.push(format!("${:.2}", value));
            }
            current_row.push(format!("=AVERAGE({}:{})",
                                     cell(1, row + 2),
                                     cell(self.previous_periods.len(), row + 2)));
        }

        current_row.push(format!("${:.2}", self.current_period));
        current_row.push(format!("${:.2}", self.goal));
        current_row.push(format!("={}-{}",
                                 cell(base_period + 2, row + 2),
                                 cell(base_period + 1, row + 2)));

        try!(writer.write(current_row.iter()));

        Ok(1)
    }
}

#[derive(Debug, Serialize)]
pub struct Budget {
    pub end_date: Date,
    period_length: Timeframe,

    period_start_dates: Vec<Date>,
    period_names: Vec<String>,
    current_period_start_date: Date,
    categories: HashMap<String, BudgetCategory>,

    income: BudgetCategory,

    any_over_budget: bool,
    has_historical: bool,
    current_period_sums: f64,
    remaining_sum: f64,
    previous_period_sums: Vec<f64>,
}

impl Budget {
    pub fn calculate(period: &Timeframe,
                     periods: usize,
                     transactions: &Transactions)
                     -> BResult<Budget> {
        let now = try! {
                transactions.date_of_last_transaction()
                    .ok_or(BudgetError::NoTransactionError)
            };
        let mut start_date = now;
        for _ in 0..periods {
            start_date -= period;
        }
        match *period {
            Weeks(_) => start_date.align_to_week(),
            Months(_) => start_date.align_to_month(),
            Quarters(_) => start_date.align_to_quarter(),
            Years(_) => start_date.align_to_year(),
            Days(_) => {},
        }

        let mut end_date = start_date + period;
        let mut ix = 0;

        let mut budget = Budget {
            period_length: *period,
            period_start_dates: vec![start_date],
            period_names: vec![format!("{} - {}", start_date, start_date + period - Days(1))],
            current_period_start_date: now,
            end_date: now,
            categories: HashMap::new(),
            has_historical: periods > 0,
            any_over_budget: false,

            income: BudgetCategory {
                name: "Income".to_owned(),
                previous_periods: vec![0.0; periods],
                current_period: 0.0,
                goal: 0.0,
            },

            current_period_sums: 0.0,
            remaining_sum: 0.0,
            previous_period_sums: Vec::new(),
        };
        /*

        factor = period / Months(1);
        for limited_category in categories::LIMITS.keys().cloned() {
            budget
                .categories
                .insert(limited_category.to_owned(),
                        BudgetCategory {
                            name: limited_category.to_owned(),
                            previous_periods: vec![0.0; periods],
                            current_period: 0.0,
                            goal: categories::LIMITS[limited_category] * factor,
                        });
        }*/

        for t in transactions.iter() {
            if t.category == "Hide" {
                continue;
            }
            if t.date >= end_date {
                ix += 1;
                start_date += period;
                end_date += period;

                if ix < periods {
                    budget.period_start_dates.push(start_date);
                    budget
                        .period_names
                        .push(format!("{} - {}", start_date, end_date - Days(1)));
                } else if ix == periods {
                    budget.current_period_start_date = start_date;
                }
            }

            if t.transaction_type == TransactionType::Debit && t.date >= start_date &&
               t.date < end_date {
                let ref mut budget_category = budget
                    .categories
                    .entry(t.category.to_owned())
                    .or_insert(BudgetCategory {
                                   name: t.category.to_owned(),
                                   previous_periods: vec![0.0; periods],
                                   current_period: 0.0,
                                   goal: 0.0,
                               });

                if ix < periods {
                    *budget_category
                         .previous_periods
                         .get_mut(ix)
                         .expect(&format!("Tried to get index {}. Too big \
                                          {}",
                                          ix,
                                          periods)) += t.amount;
                } else if ix == periods {
                    budget_category.current_period += t.amount;
                }
            } else if t.transaction_type == TransactionType::Credit && t.date >= start_date &&
                      t.date < end_date {

                if ix < periods {
                    *budget
                         .income
                         .previous_periods
                         .get_mut(ix)
                         .expect(&format!("Tried to get index {}. Too big \
                                          {}",
                                          ix,
                                          periods)) += t.amount;
                } else if ix == periods {
                    budget.income.current_period += t.amount;
                }

            }
        }

        budget.any_over_budget = budget
            .categories
            .iter()
            .filter(|&(_, c)| c.current_period > c.goal)
            .count() > 0;

        budget.current_period_sums = budget
            .categories
            .iter()
            .fold(0.0, |acc, (&_, ref c)| acc + c.current_period);

        budget.remaining_sum = budget
            .categories
            .iter()
            .fold(0.0, |acc, (&_, ref c)| acc + c.goal - c.current_period);

        budget.previous_period_sums = budget
            .categories
            .iter()
            .fold(vec![0.0; periods], |acc, (&_, ref c)| {
                acc.iter()
                    .enumerate()
                    .map(|(ix, &val)| c.previous_periods[ix] + val)
                    .collect()
            });

        Ok(budget)
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> BResult<usize> {
        let mut keys: Vec<_> = self.categories.keys().collect();
        let mut outfile = try!(csv::Writer::from_file(path));
        keys.sort();

        let mut current_row = Vec::new();
        current_row.push("Category".to_owned());
        if self.has_historical {
            for period_start in &self.period_start_dates {
                current_row.push(format!("{} - {}",
                                         period_start,
                                         period_start + self.period_length - Days(1)));
            }
            current_row.push("Average".to_owned());
        }
        let base_period = if self.has_historical {
            self.period_start_dates.len() + 1
        } else {
            0
        };
        current_row.push(format!("{} - {}", self.current_period_start_date, self.end_date));
        current_row.append(&mut vec!["Target Budget".to_owned(), "Budget Left".to_owned()]);
        try!(outfile.write(current_row.iter()));
        for (row, category_name) in keys.iter().enumerate() {
            if **category_name != "Income".to_owned() {
                current_row.clear();

                try!(self.categories[*category_name].write_to_file(row, &mut outfile));
            }
        }

        current_row.clear();
        current_row.push("Total".to_owned());
        for i in 0..base_period + 3 {
            current_row.push(format!("=sum({}:{})", cell(i + 1, 2), cell(i + 1, keys.len() + 1)));
        }
        try!(outfile.write(current_row.iter()));

        current_row.clear();
        current_row.push("".to_owned());
        for _ in 0..base_period + 3 {
            current_row.push("".to_owned());
        }
        try!(outfile.write(current_row.iter()));
        try!(outfile.write(current_row.iter()));

        try!(self.income.write_to_file(keys.len() + 3, &mut outfile));

        current_row.clear();
        current_row.push("Money Saved".to_owned());
        for i in 0..base_period + 3 {
            current_row.push(format!("={} - {}",
                                     cell(i + 1, keys.len() + 5),
                                     cell(i + 1, keys.len() + 2)));
        }
        try!(outfile.write(current_row.iter()));

        Ok(self.categories.len())
    }
}

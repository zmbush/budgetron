// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    loading::Transaction,
    num_traits::Float,
    ordered_float::NotNaN,
    reporting::Reporter,
    serde_json::{self, Value},
    std::{borrow::Cow, collections::BTreeMap},
};

pub struct RepeatedTransactions {
    threshold: f64,
}

impl RepeatedTransactions {
    pub fn new(threshold: f64) -> RepeatedTransactions {
        RepeatedTransactions { threshold }
    }
}

#[derive(Debug, Serialize)]
struct Report {
    transactions: Vec<Transaction>,
    average_date_delta: i64,
}

#[derive(Debug)]
struct NearbyFloatsBucket<F: Float> {
    high: NotNaN<F>,
    low: NotNaN<F>,
}

impl<F: Float> NearbyFloatsBucket<F> {
    fn from(low: NotNaN<F>, high: NotNaN<F>) -> NearbyFloatsBucket<F> {
        NearbyFloatsBucket { high, low }
    }
}

struct NearbyFloatsBuilder<F: Float> {
    margin: F,
    buckets: Vec<(NotNaN<F>, NotNaN<F>)>,
}

impl<F: Float> NearbyFloatsBuilder<F> {
    fn new() -> NearbyFloatsBuilder<F> {
        NearbyFloatsBuilder {
            margin: F::epsilon(),
            buckets: Vec::new(),
        }
    }

    fn margin(mut self, margin: F) -> NearbyFloatsBuilder<F> {
        self.margin = margin;
        self
    }

    fn seed(mut self, mut floats: Vec<F>) -> NearbyFloatsBuilder<F> {
        floats.sort_by(|a, b| NotNaN::from(*a).cmp(&NotNaN::from(*b)));
        let mut low = floats[0];
        let mut previous = floats[0];
        for float in floats {
            if float > previous + self.margin {
                self.buckets
                    .push((NotNaN::from(low), NotNaN::from(previous)));
                low = float;
            }
            previous = float;
        }
        self.buckets
            .push((NotNaN::from(low), NotNaN::from(previous)));

        self
    }

    fn build(self) -> NearbyFloats<F> {
        NearbyFloats {
            buckets: self
                .buckets
                .into_iter()
                .map(|(low, high)| NearbyFloatsBucket::from(low, high))
                .collect(),
        }
    }
}

#[derive(Debug)]
struct NearbyFloats<F: Float> {
    buckets: Vec<NearbyFloatsBucket<F>>,
}

impl Reporter for RepeatedTransactions {
    fn report<'a, T>(&self, transactions: T) -> Value
    where
        T: Iterator<Item = Cow<'a, Transaction>> + Clone,
    {
        let finder = NearbyFloatsBuilder::new()
            .margin(10.)
            .seed(transactions.clone().map(|t| t.amount).collect())
            .build();
        println!("buckets: {:?}", finder);
        let mut seen = BTreeMap::new();
        for transaction in transactions {
            let transaction = transaction.into_owned();
            if transaction.amount < self.threshold {
                continue;
            }
            (*seen
                .entry((
                    transaction.description.clone(),
                    transaction.transaction_type,
                ))
                .or_insert_with(|| Vec::new()))
            .push(transaction);
        }
        let seen = seen
            .into_iter()
            .filter_map(|((amt, transaction_type), transactions)| {
                if transactions.len() > 2 {
                    let (count, total) = transactions
                        .windows(2)
                        .map(|w| w[1].date - w[0].date)
                        .fold((0, 0), |curr, d| (curr.0 + 1, curr.1 + d));
                    Some((
                        format!("{:?} {:?}", transaction_type, amt),
                        Report {
                            transactions,
                            average_date_delta: total / count,
                        },
                    ))
                } else {
                    None
                }
            })
            .collect::<BTreeMap<_, _>>();
        serde_json::to_value(seen).expect("couldn't convert")
    }

    fn key(&self) -> Option<String> {
        Some("repeated_transactions".to_owned())
    }
}

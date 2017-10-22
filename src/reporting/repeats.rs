use loading::Transaction;
use num_traits::Float;
use ordered_float::NotNaN;
use reporting::Reporter;
use serde_json::{self, Value};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;

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
        NearbyFloatsBuilder { margin: F::epsilon(), buckets: Vec::new() }
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
                self.buckets.push((NotNaN::from(low), NotNaN::from(previous)));
                low = float;
            }
            previous = float;
        }
        self.buckets.push((NotNaN::from(low), NotNaN::from(previous)));

        self
    }

    fn build(self) -> NearbyFloats<F> {
        NearbyFloats {
            buckets: self.buckets
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

impl<F: Float + fmt::Debug> NearbyFloats<F> {
    fn find(&mut self, num: F) -> Option<NotNaN<F>> {
        let num = NotNaN::from(num);
        let result = self.buckets.binary_search_by(|bucket| if bucket.low > num {
            Ordering::Greater
        } else if bucket.high < num {
            Ordering::Less
        } else {
            Ordering::Equal
        });
        match result {
            Ok(index) => {
                let bucket = self.buckets.get_mut(index).expect("Binary search lied");
                if num < bucket.low {
                    bucket.low = num;
                }
                if num > bucket.high {
                    bucket.high = num;
                }
                Some(bucket.low)
            },
            Err(_) => None,
        }
    }
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
            (*seen.entry((transaction.description.clone(), transaction.transaction_type))
                 .or_insert_with(|| Vec::new()))
                .push(transaction);
        }
        let seen = seen.into_iter()
            .filter_map(|((amt, transaction_type), transactions)| if transactions.len() > 2 {
                let (count, total) = transactions.windows(2).map(|w| w[1].date - w[0].date).fold(
                    (0, 0),
                    |curr, d| (curr.0 + 1, curr.1 + d),
                );
                Some((
                    format!("{:?} {:?}", transaction_type, amt),
                    Report { transactions, average_date_delta: total / count },
                ))
            } else {
                None
            })
            .collect::<BTreeMap<_, _>>();
        serde_json::to_value(seen).expect("couldn't convert")
    }

    fn key(&self) -> Option<String> {
        Some("repeated_transactions".to_owned())
    }
}

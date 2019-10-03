// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use budgetronlib::fintime::Date;
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct Timeseries<V>(Vec<TimeseriesDatum<V>>);

#[derive(Debug, Serialize)]
pub struct TimeseriesDatum<V> {
    date: Date,
    value: V,
}

impl<V> Timeseries<V> {
    pub fn new() -> Timeseries<V> {
        Timeseries(Vec::new())
    }

    pub fn add(&mut self, date: Date, value: V) {
        if let Some(datum) = self.0.last_mut() {
            if datum.date == date {
                datum.value = value;
                return;
            }
        }
        self.0.push(TimeseriesDatum { date, value });
    }
}

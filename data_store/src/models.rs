// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use chrono::NaiveDate;

#[derive(Queryable)]
pub struct Transaction {
    pub id: i32,
    pub date: NaiveDate,
    pub person: String,
    pub description: String,
    pub original_description: String,
    pub amount: f32,
    pub transaction_type: String,
    pub category: String,
    pub original_category: String,
    pub account_name: String,
    pub labels: String,
    pub notes: String,
    pub transfer_destination_account: Option<String>,
    pub tags: Vec<String>,
}

use super::schema::transactions;

#[derive(Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction<'a> {
    pub date: NaiveDate,
    pub person: String,
    pub description: String,
    pub original_description: String,
    pub amount: f64,
    pub transaction_type: &'a str,
    pub category: String,
    pub original_category: String,
    pub account_name: String,
    pub labels: String,
    pub notes: String,
    pub transfer_destination_account: Option<String>,
    pub tags: Vec<String>,
}

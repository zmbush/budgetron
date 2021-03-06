// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![recursion_limit = "128"]
#![deny(unused, unused_extern_crates)]

#[macro_use]
extern crate diesel_codegen;

pub mod models;
pub mod schema;

extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate itertools;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use itertools::Itertools;
use std::env;

pub struct Transactions {
    pub db: PgConnection,
}

impl Transactions {
    pub fn new_from_env() -> Transactions {
        let _ = dotenv();

        Transactions::new(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
    }

    pub fn new(database_url: &str) -> Transactions {
        Transactions {
            db: PgConnection::establish(database_url)
                .expect(&format!("Error connecting to {}", database_url)),
        }
    }

    pub fn set_transactions(&self, transactions: Vec<models::NewTransaction>) {
        use schema::transactions;

        diesel::delete(transactions::table)
            .execute(&self.db)
            .expect("Unable to delete the old transactions table");
        for group in &transactions.into_iter().chunks(1000) {
            let group = group.collect::<Vec<models::NewTransaction>>();
            diesel::insert(&group)
                .into(transactions::table)
                .execute(&self.db)
                .expect("Error saving transaction");
        }
    }
}

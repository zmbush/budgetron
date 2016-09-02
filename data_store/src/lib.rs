#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

use diesel::prelude::*;
use diesel::pg::{Pg, PgConnection};
use dotenv::dotenv;
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

    pub fn set_transactions(&self, transactions: &Vec<models::NewTransaction>) {
        use schema::transactions;

        diesel::delete(transactions::table)
            .execute(&self.db)
            .expect("Unable to delete the old transactions table");
        diesel::insert(transactions)
            .into(transactions::table)
            .execute(&self.db)
            .expect("Error saving transaction");
    }
}

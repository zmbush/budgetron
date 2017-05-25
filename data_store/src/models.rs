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
}

use super::schema::transactions;

#[derive(Insertable)]
#[table_name="transactions"]
pub struct NewTransaction<'a> {
    pub date: NaiveDate,
    pub person: &'a str,
    pub description: &'a str,
    pub original_description: &'a str,
    pub amount: f64,
    pub transaction_type: &'a str,
    pub category: &'a str,
    pub original_category: &'a str,
    pub account_name: &'a str,
    pub labels: &'a str,
    pub notes: &'a str,
    pub transfer_destination_account: Option<&'a String>,
}

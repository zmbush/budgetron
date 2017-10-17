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

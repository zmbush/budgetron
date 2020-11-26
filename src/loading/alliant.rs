// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    crate::loading::{
        generic::{Genericize, Transaction, TransactionType},
        money::Money,
    },
    budgetronlib::{error::BResult, fintime::Date},
    serde::Deserialize,
};

// "id","account_id","reference_id","transaction_type","amount","posted_at",
// "created_at","nickname","original_name","merchant_id","updated_at",
// "check_number","account_name","tags"
#[derive(Debug, Deserialize)]
pub enum AlliantTransactionType {
    Debit,
    Credit,
}

impl Into<TransactionType> for AlliantTransactionType {
    fn into(self) -> TransactionType {
        match self {
            AlliantTransactionType::Debit => TransactionType::Debit,
            AlliantTransactionType::Credit => TransactionType::Credit,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AlliantExport1 {
    id: String,
    account_id: i64,
    reference_id: i64,
    transaction_type: AlliantTransactionType,
    amount: Money,
    posted_at: Date,
    created_at: Date,
    nickname: String,
    original_name: String,
    merchant_id: String,
    updated_at: Date,
    check_number: Option<i32>,
    account_name: String,
    tags: String,
}

impl Genericize for AlliantExport1 {
    fn genericize(self) -> BResult<Transaction> {
        Ok(Transaction {
            uid: Some(self.id),
            date: self.posted_at,
            person: "".to_owned(),
            description: self.nickname,
            original_description: self.original_name,
            amount: self.amount,
            transaction_type: self.transaction_type.into(),
            category: self.tags.clone(),
            original_category: self.tags,
            account_name: self.account_name,
            labels: "".to_owned(),
            notes: "".to_owned(),
            transfer_destination_account: None,
            tags: vec![],
        })
    }
}

fn parse_alliant_date<'de, D>(data: D) -> Result<Date, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use {chrono::Datelike, serde::de, std::fmt};

    struct DateVisitor;
    impl<'de> de::Visitor<'de> for DateVisitor {
        type Value = Date;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a string containing a date in rfc3339 style")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let date = chrono::DateTime::parse_from_rfc3339(s);
            match date {
                Err(e) => Err(de::Error::custom(format!("Bad date format: {}", e))),
                Ok(d) => Ok(Date::ymd(
                    d.date().year(),
                    d.date().month() as i32,
                    d.date().day() as i32,
                )),
            }
        }
    }

    data.deserialize_str(DateVisitor)
}

// id,account_id,reference_id,transaction_type,amount,posted_at,created_at,nickname,original_name,check_number,account_name,tags
#[derive(Debug, Deserialize)]
pub struct AlliantExport2 {
    id: String,
    account_id: i64,
    reference_id: i64,
    transaction_type: AlliantTransactionType,
    amount: Money,
    #[serde(deserialize_with = "parse_alliant_date")]
    posted_at: Date,
    #[serde(deserialize_with = "parse_alliant_date")]
    created_at: Date,
    nickname: String,
    original_name: String,
    check_number: Option<i32>,
    account_name: String,
    tags: String,
}

impl Genericize for AlliantExport2 {
    fn genericize(self) -> BResult<Transaction> {
        Ok(Transaction {
            uid: Some(self.id),
            date: self.posted_at,
            person: "".to_owned(),
            description: self.nickname,
            original_description: self.original_name,
            amount: self.amount,
            transaction_type: self.transaction_type.into(),
            category: self.tags.clone(),
            original_category: self.tags,
            account_name: self.account_name,
            labels: "".to_owned(),
            notes: "".to_owned(),
            transfer_destination_account: None,
            tags: vec![],
        })
    }
}

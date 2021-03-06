use crate::schema::{accounts, transactions};

#[derive(Queryable)]
pub struct Account {
    pub id: i32,
    pub account_name: String,
    pub account_number: String,
    pub account_holder: String,
    pub balance: f32,
}

#[derive(Debug, Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub account_name: &'a str,
    pub account_number: &'a str,
    pub account_holder: &'a str,
    pub balance: f32,
}

#[derive(Queryable)]
pub struct Transaction {
    pub id: i32,
    pub account_name: String,
    pub account_number: String,
    pub locale: String,
    pub occurrance: String,
    pub amount: f32,
}

#[derive(Debug, Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction<'a> {
    pub account_name: &'a str,
    pub account_number: &'a str,
    pub locale: &'a str,
    pub occurrance: &'a str,
    pub amount: f32,
}

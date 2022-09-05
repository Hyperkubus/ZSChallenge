use diesel::{prelude::*};
use crate::schema::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Account {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, AsChangeset)]
#[table_name="accounts"]
pub struct NewAccount<'a> {
    pub email: &'a String,
    pub password: &'a String,
    pub firstname: &'a String,
    pub lastname: &'a String,
}

#[derive(Queryable, Serialize)]
pub struct BankDetail {
    pub id: i32,
    pub account_id: i32,
    pub holder: String,
    pub iban: String,
    pub bic: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, AsChangeset)]
#[table_name="bankdetails"]
pub struct NewBankDetail<'a> {
    pub account_id: &'a i32,
    pub holder: &'a String,
    pub iban: &'a String,
    pub bic: &'a String,
}

#[derive(Queryable, Serialize)]
pub struct CarDetail {
    pub id: i32,
    pub account_id: i32,
    pub owner: String,
    pub plate: String,
    pub registration: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, AsChangeset)]
#[table_name="cardetails"]
pub struct NewCarDetail<'a> {
    pub account_id: &'a i32,
    pub owner: &'a String,
    pub plate: &'a String,
    pub registration: &'a String,
}
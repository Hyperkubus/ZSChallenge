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
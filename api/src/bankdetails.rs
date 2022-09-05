use crate::establish_connection;
use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use rocket::*;
use rocket_contrib::json::Json;
use serde::Deserialize;

#[get("/bankdetail")]
pub fn list() -> Json<Vec<BankDetails>> {
    use crate::schema::bankdetails::dsl::*;
    let bankdetail_list = bankdetails
        .filter(deleted_at.is_null())
        .load::<BankDetail>(&mut establish_connection())
        .expect("Error fetching Account");
    Json(bankdetail_list)
}

#[get("/bankdetail/<bankdetail_id>")]
pub fn get(bankdetail_id: i32) -> Json<BankDetail> {
    use crate::schema::bankdetail::dsl::*;
    let bank_detail = bankdetails
        .select(bankdetails::all_columns())
        .filter(id.eq(bankdetail_id))
        .filter(deleted_at.is_null())
        .get_result::<BankDetail>(&mut establish_connection())
        .expect("Error fetching Account");
    Json(bank_detail)
}


#[derive(Deserialize)]
pub struct PostBankDetails {
    pub account_id: String,
    pub holder: String,
    pub iban: String,
    pub bic: String,
}

#[post("/account", format = "json", data = "<post_account>")]
pub fn new_account(post_account: Json<PostAccount>) {
    let new_account = NewAccount {
        account_id: &post_account.email,
        password: &post_account.password,
        firstname: &post_account.firstname,
        lastname: &post_account.lastname
    };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .execute(&mut establish_connection())
        .expect("Error saving new account");
}
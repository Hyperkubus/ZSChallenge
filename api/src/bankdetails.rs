use crate::establish_connection;
use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use rocket::*;
use rocket_contrib::json::Json;
use serde::Deserialize;

#[get("/bankdetail")]
pub fn list() -> Json<Vec<BankDetail>> {
    use crate::schema::bankdetails::dsl::*;
    let bankdetail_list = bankdetails
        .filter(deleted_at.is_null())
        .load::<BankDetail>(&mut establish_connection())
        .expect("Error fetching Account");
    Json(bankdetail_list)
}

#[get("/bankdetail/<bankdetail_id>")]
pub fn get(bankdetail_id: i32) -> Json<BankDetail> {
    use crate::schema::bankdetails::dsl::*;
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
    pub account_id: i32,
    pub holder: String,
    pub iban: String,
    pub bic: String,
}

#[post("/bankdetail", format = "json", data = "<post_details>")]
pub fn new(post_details: Json<PostBankDetails>) {
    let new_details = NewBankDetail {
        account_id: &post_details.account_id,
        holder: &post_details.holder,
        iban: &post_details.iban,
        bic: &post_details.bic
    };

    diesel::insert_into(bankdetails::table)
        .values(&new_details)
        .execute(&mut establish_connection())
        .expect("Error saving new details");
}

#[delete("/bankdetail/<bankdetail_id>")]
pub fn delete(bankdetail_id: i32) -> Json<String> {
    use crate::schema::bankdetails::dsl::*;
    diesel::delete(bankdetails.filter(id.eq(bankdetail_id))).execute(&mut establish_connection())
    .expect("Error deleting details");
    Json(String::from("record deleted"))
}
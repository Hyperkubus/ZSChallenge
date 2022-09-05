use crate::establish_connection;
use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use rocket::*;
use rocket_contrib::json::Json;
use serde::Deserialize;

#[get("/cardetail")]
pub fn list() -> Json<Vec<CarDetail>> {
    use crate::schema::cardetails::dsl::*;
    let cardetail_list = cardetails
        .filter(deleted_at.is_null())
        .load::<CarDetail>(&mut establish_connection())
        .expect("Error fetching Account");
    Json(cardetail_list)
}

#[get("/cardetail/<cardetail_id>")]
pub fn get(cardetail_id: i32) -> Json<CarDetail> {
    use crate::schema::cardetails::dsl::*;
    let car_detail = cardetails
        .select(cardetails::all_columns())
        .filter(id.eq(cardetail_id))
        .filter(deleted_at.is_null())
        .get_result::<CarDetail>(&mut establish_connection())
        .expect("Error fetching Account");
    Json(car_detail)
}


#[derive(Deserialize)]
pub struct PostCarDetails {
    pub account_id: i32,
    pub owner: String,
    pub plate: String,
    pub registration: String,
}

#[post("/cardetail", format = "json", data = "<post_details>")]
pub fn new(post_details: Json<PostCarDetails>) {
    let new_details = NewCarDetail {
        account_id: &post_details.account_id,
        owner: &post_details.owner,
        plate: &post_details.plate,
        registration: &post_details.registration
    };

    diesel::insert_into(cardetails::table)
        .values(&new_details)
        .execute(&mut establish_connection())
        .expect("Error saving new details");
}

#[delete("/cardetail/<cardetail_id>")]
pub fn delete(cardetail_id: i32) -> Json<String> {
    use crate::schema::cardetails::dsl::*;
    diesel::delete(cardetails.filter(id.eq(cardetail_id))).execute(&mut establish_connection())
    .expect("Error deleting details");
    Json(String::from("record deleted"))
}
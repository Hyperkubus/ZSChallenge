use crate::establish_connection;
use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use rocket::*;
use rocket_contrib::json::Json;
use serde::Deserialize;

#[get("/account")]
pub fn list() -> Json<Vec<Account>> {
    use crate::schema::accounts::dsl::*;
    let user_accounts = accounts
        .filter(deleted_at.is_null())
        .load::<Account>(&mut establish_connection())
        .expect("Error fetching Account");
    Json(user_accounts)
}

#[get("/account/<account_id>")]
pub fn get(account_id: i32) -> Json<Account> {
    use crate::schema::accounts::dsl::*;
    let user_account = accounts
        .select(accounts::all_columns())
        .filter(id.eq(account_id))
        .filter(deleted_at.is_null())
        .get_result::<Account>(&mut establish_connection())
        .expect("Error fetching Account");
    Json(user_account)
}

#[derive(Deserialize)]
pub struct PostAccount {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[post("/account", format = "json", data = "<post_account>")]
pub fn new(post_account: Json<PostAccount>) {
    let new_account = NewAccount {
        email: &post_account.email,
        password: &post_account.password,
        firstname: &post_account.firstname,
        lastname: &post_account.lastname,
    };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .execute(&mut establish_connection())
        .expect("Error saving new account");
}

#[delete("/account/<account_id>")]
pub fn delete(account_id: i32) -> Json<String> {
    use crate::schema::accounts::dsl::*;
    diesel::delete(accounts.filter(id.eq(account_id)))
        .execute(&mut establish_connection())
        .expect("Error deleting details");
    Json(String::from("record deleted"))
}

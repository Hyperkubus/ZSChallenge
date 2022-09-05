#![feature(proc_macro_hygiene, decl_macro)]

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::*;
use std::env;

pub mod models;
pub mod schema;
pub mod accounts;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("ENVIRONMENT must be set");
    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}


#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

fn main() {
    dotenv().ok();
    let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT must be set");

    println!("Starting server in environment: {}", environment);
    
    rocket::ignite()
    .mount("/", routes![
        index,
        accounts::list,
        accounts::get_account,
        accounts::new_account
    ])
    .launch();
}

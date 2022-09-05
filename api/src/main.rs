#![feature(proc_macro_hygiene, decl_macro)]

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::*;
use std::env;

pub mod models;
pub mod schema;
pub mod auth;
pub mod accounts;
pub mod bankdetails;
pub mod cardetails;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("ENVIRONMENT must be set");
    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}


fn main() {
    dotenv().ok();
    let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT must be set");

    println!("Starting server in environment: {}", environment);
    
    rocket::ignite()
    .mount("/", routes![
        //auth::login,
        accounts::list,
        accounts::get,
        accounts::new,
        accounts::delete,
        bankdetails::list,
        bankdetails::get,
        bankdetails::new,
        bankdetails::delete,
        cardetails::list,
        cardetails::get,
        cardetails::new,
        cardetails::delete
    ])
    .launch();
}

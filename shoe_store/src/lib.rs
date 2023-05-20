#[macro_use]

extern crate diesel;

extern crate dotenv;

extern crate serde;

extern crate serde_json;



pub mod schema;

pub mod models;



use diesel::prelude::*;

use diesel::sqlite::SqliteConnection;

use dotenv::dotenv;

use std::env;



pub fn establish_connection() -> SqliteConnection {

    dotenv().ok();


    let database_url = env::var("DATABASE_URL")

        .expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)

        .expect(&format!("Error connecting to {}", database_url))

}



pub fn establish_connection_test() -> SqliteConnection {

    dotenv().ok();



    let database_url = env::var("TEST_DATABASE_URL")

        .expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)

        .expect(&format!("Error connecting to {}", database_url))

}
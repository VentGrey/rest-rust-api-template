use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

mod db;
mod models;
mod routes;
mod schema;
mod static_files;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let pool = db::init_pool(db_url);
    rocket::build().mount(
        "/",
        routes![crate::static_files::all, crate::static_files::index],
    )
}

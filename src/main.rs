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

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let pool = db::init_pool(db_url);
    rocket::build()
        .manage(pool)
        .mount(
            "/api/",
            routes![
                crate::routes::index,
                crate::routes::new,
                crate::routes::show,
                crate::routes::delete,
                crate::routes::name,
                crate::routes::update
            ],
        )
        .mount(
            "/",
            routes![crate::static_files::all, crate::static_files::index],
        )
}

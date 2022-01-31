use dotenv::dotenv;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
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

struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let pool = db::init_pool(db_url);
    rocket::build()
        .manage(pool)
        .attach(CORS)
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

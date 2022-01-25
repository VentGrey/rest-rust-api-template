use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[derive(Queryable)]
pub struct Cat {
    pub id: i32,
    pub name: String,
    pub photo_url: String,
    pub is_adopted: bool,
    pub description: String,
}

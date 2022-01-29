use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::schema::cats;
use crate::schema::cats::dsl::cats as all_cats;

#[derive(Queryable)]
pub struct Cat {
    pub id: i32,
    pub name: String,
    pub photo_url: String,
    pub is_adopted: bool,
    pub description: String,
}

#[derive(Insertable)]
#[table_name = "cats"]
pub struct NewCat {
    pub name: String,
    pub photo_url: String,
    pub is_adopted: bool,
    pub description: String,
}

impl Cat {
    pub fn show(id: i32, conn: &SqliteConnection) -> Vec<Cat> {
        all_cats
            .find(id)
            .load::<Cat>(conn)
            .expect("Ocurrió un error al cargar el gato...")
    }

    pub fn all(conn: &SqliteConnection) -> Vec<Cat> {
        all_cats
            .order(cats::id.desc())
            .load::<Cat>(conn)
            .expect("Ocurrió un error al cargar todos los gatos...")
    }

    pub fn update_by_id(id: i32, conn: &SqliteConnection, cat: NewCat) -> bool {
        use crate::schema::cats::dsl::{
            description as d, is_adopted as i, name as n, photo_url as p,
        };

        let NewCat {
            name,
            photo_url,
            is_adopted,
            description,
        } = cat;

        diesel::update(all_cats.find(id))
            .set((
                n.eq(name),
                p.eq(photo_url),
                i.eq(is_adopted),
                d.eq(description),
            ))
            .execute(conn)
            .is_ok()
    }
}

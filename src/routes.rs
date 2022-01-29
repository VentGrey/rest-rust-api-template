use crate::db::Conn as DbConn;
use crate::models::{Cat, NewCat};
use rocket::serde::json::{json, Json, Value};

#[get("/cats", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let cats: Vec<Cat> = Cat::all(&conn);

    Json(json!({
        "status": 200,
        "result": cats,
    }))
}

#[post("/cats", format = "application/json", data = "<new_cat>")]
pub fn new(conn: DbConn, new_cat: Json<NewCat>) -> Json<Value> {
    Json(json!({
        "status": Cat::insert(new_cat.into_inner(), &conn),
        "result": Cat::all(&conn).first(),
    }))
}

#[get("/cats/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result: Vec<Cat> = Cat::show(id, &conn);
    let status: i32 = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}

#[put("/cats/<id>", format = "application/json", data = "<cat>")]
pub fn update(conn: DbConn, id: i32, cat: Json<NewCat>) -> Json<Value> {
    let status: i32 = if Cat::update_by_id(id, &conn, cat.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/cats/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status: i32 = if Cat::delete_by_id(id, &conn) {
        200
    } else {
        404
    };
    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[get("/cats/names/<name>", format = "application/json")]
pub fn name(name: String, conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": Cat::all_by_name(name, &conn),
    }))
}

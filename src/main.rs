#[macro_use]
extern crate rocket;
use rocket::{response::Responder, serde::json::Json};

mod http;
mod kv;
use http::*;

#[get("/<id>")]
fn get(id: String) -> Json<Response<String>> {
    KVResponse::new("OK".to_string(), "salam".to_string())
}

#[post("/")]
fn set(id: String) -> Json<Response<String>> {}

#[delete("/<id>")]
fn del(id: String) -> Json<Response<String>> {}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![get, set, del])
}

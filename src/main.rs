#[macro_use]
extern crate rocket;
use kv::Storage;
use rocket::{response::Responder, serde::json::Json, State};

mod kv;

#[get("/<id>")]
fn get(storage: &State<kv::HashMapStorage<String, String>>, id: String) -> Json<String> {
    match storage.get(id) {
        Ok(v) => Json(v.to_string()),
        Err(e) => Json(format!("{}", e)),
    }
}

#[put("/<id>/<value>")]
fn set(
    storage: &State<kv::HashMapStorage<String, String>>,
    id: String,
    value: String,
) -> Json<String> {
    let res = storage.put(id, value);
    match res {
        Ok(_) => Json("OK".to_string()),
        Err(e) => Json(format!("{}", e)),
    }
}

#[delete("/<id>")]
fn del(storage: &State<kv::HashMapStorage<String, String>>, id: String) -> Json<String> {
    match storage.del(id) {
        Ok(_) => Json("Deleted".to_string()),
        Err(e) => Json(format!("{}", e)),
    }
}

#[launch]
fn launch() -> _ {
    rocket::build()
        .manage(kv::HashMapStorage::<String, String>::new())
        .mount("/", routes![get, set, del])
}

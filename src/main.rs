#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::env;
use rocket::http::RawStr;
use rocket::State;
use std::collections::HashMap;

struct Store<'a> {
    store: HashMap<& 'a String, String>,
}

#[get("/store/<id>")]
fn retrieve(id: &RawStr, store: State<Store>) -> String {
    format!("{}: {:?}", id, store.store.get(&id.to_string()))
}

#[get("/store/<id>/<val>")]
fn set(id: &RawStr, val: &RawStr, store: State<Store>) -> String {
    let id_copy = id.to_string();
    let value = val.to_string();
    store.store.insert(&id_copy, value);
    format!("{}: {:?}", id, store.store.get(&id.to_string()))
}

fn main() {
    match env::var("PORT") {
        Ok(val) => env::set_var("ROCKET_PORT", val),
        Err(e) => println!("Error: {}", e),
    }
    rocket::ignite()
        .manage(Store { store: HashMap::new()})
        .mount("/", routes![retrieve])
        .launch();
}

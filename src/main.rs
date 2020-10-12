#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::env;
use rocket::http::RawStr;
use rocket::State;
use std::collections::HashMap;
use std::sync::RwLock;

struct Store<> {
    store: RwLock<HashMap<String, String>>,
}

#[get("/store/<id>")]
fn retrieve(id: &RawStr, store: State<Store>) -> String {
    let _astring = "test";
    let hm = store.store.read().unwrap();
    format!("{}: {:?}", id, hm.get(&id.to_string()))
}

#[get("/store/<id>/<val>")]
fn set(id: &RawStr, val: &RawStr, store: State<Store>) -> String {
    let id_copy = id.to_string();
    let value = val.to_string();

    let mut hm = store.store.write().unwrap();
    hm.insert(id_copy, value);
    drop(hm);
    let hm_read = store.store.read().unwrap();
    format!("{}: {:?}", id, hm_read.get(&id.to_string()))
}

fn main() {
    match env::var("PORT") {
        Ok(val) => env::set_var("ROCKET_PORT", val),
        Err(e) => println!("Error: {}", e),
    }
    rocket::ignite()
        .manage(Store { store: RwLock::new(HashMap::new())})
        .mount("/", routes![retrieve, set])
        .launch();
}

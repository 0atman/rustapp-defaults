#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! rocket = "0.4.4"
//! ```
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::env;

#[get("/store/<id>")]
fn retrieve(id: String) -> String {
    match simple_redis::create("redis://127.0.0.1:6379/") {
        Ok(mut client) => {
            match client.get_string(&id) {
                Ok(value) => value,
                Err(error) => error.to_string(),
            }
        }
        Err(error) => "redis error".to_string() + &error.to_string(),
    }
}

#[get("/store/<id>/<val>")]
fn set(id: String, val: String) -> String {
    match simple_redis::create("redis://127.0.0.1:6379/") {
        Ok(mut client) => {
            match client.set(&id, &*val) {
                Err(error) => error.to_string(),
                _ => "Value set in Redis".to_string(),
            }
        }
        Err(error) => error.to_string(),
    }
}

fn main() {
    match env::var("PORT") {
        Ok(val) => env::set_var("ROCKET_PORT", val),
        Err(e) => println!("Error: {}", e),
    }
    rocket::ignite()
        .mount("/", routes![retrieve, set])
        .launch();
}

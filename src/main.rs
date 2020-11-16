#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! rocket = "0.4.4"
//! ```
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::env;
use mini_redis::{client, Result};
use rocket::http::RawStr;

#[get("/store/<id>")]
async fn retrieve(id: &RawStr) -> Result<String> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    let result = client.get(&id.to_string()).await?;
    Ok(format!("{}", id))
}

#[get("/store/<id>/<val>")]
fn set(id: &RawStr, val: &RawStr) -> String {
    let value= val.to_string();

    format!("{}: {:?}", id, value)
}

#[tokio::main]
async fn main() -> Result<()> {
    match env::var("PORT") {
        Ok(val) => env::set_var("ROCKET_PORT", val),
        Err(e) => println!("Error: {}", e),
    }
    rocket::ignite()
        .mount("/", routes![retrieve, set])
        .launch();
    Ok(())
}

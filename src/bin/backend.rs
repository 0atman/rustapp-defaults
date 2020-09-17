#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/tasks")]
fn tasks_get() -> String {
    "this is a response\n".into()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![tasks_get])
        .launch();
}

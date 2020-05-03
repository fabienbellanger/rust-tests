#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate chrono;

use serde::Serialize;
use rocket_contrib::json::Json;
use chrono::prelude::*;

#[derive(Serialize)]
struct Task {
    id: u32,
    name: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/json")]
fn json() -> Json<Task> {
    Json(Task{id: 12, name: "Coucou".to_owned()})
}

#[get("/time")]
fn time_now() -> String {
    let utc: DateTime<Utc> = Utc::now();

    utc.to_rfc3339()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, json, time_now])
        .launch();
}

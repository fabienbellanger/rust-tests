#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use serde::Serialize;
use rocket_contrib::json::Json;

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

fn main() {
    rocket::ignite()
        .mount("/", routes![index, json])
        .launch();
}

extern crate chrono;

use rocket_contrib::json::Json;
use serde::Serialize;
use chrono::prelude::*;
use std::time::Duration;

#[derive(Serialize)]
pub struct Task {
    id: u32,
    name: &'static str,
    message: String,
}

#[get("/")]
pub fn index() -> &'static str {
    std::thread::sleep(Duration::from_secs(1));
    "Hello, world!"
}

#[get("/json")]
pub fn json() -> Json<Task> {
    Json(Task{id: 12, name: "Coucou", message: String::from("Mon message")})
}

#[get("/time")]
pub fn time_now() -> String {
    let now: DateTime<Utc> = Utc::now();

    now.to_rfc3339_opts(SecondsFormat::Secs, true)
}

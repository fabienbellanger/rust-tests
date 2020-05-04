#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod handler;
mod fairing;

fn main() {
    rocket::ignite()
        .attach(fairing::RequestTimer)
        .mount("/", routes![handler::index, handler::json, handler::time_now])
        .launch();
}

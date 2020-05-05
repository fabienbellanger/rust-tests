#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

// use rocket_contrib::databases::diesel;

mod handler;
mod fairing;
mod error;

// #[database("mysql")]
// struct MyDatabase(diesel::MysqlConnection);

fn main() {
    rocket::ignite()
        .attach(fairing::RequestTimer)
        // .attach(MyDatabase::fairing())
        .mount("/", routes![
            handler::index, 
            handler::json, 
            handler::big_json, 
            handler::time_now,
        ])
        .register(
            catchers![
                error::not_found,
                error::internal_server_error],
        )
        .launch();
}

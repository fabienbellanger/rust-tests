#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::databases::diesel;

mod handler;
mod fairing;
mod error;
mod models;

#[database("mysql")]
struct MyDatabase(diesel::MysqlConnection);

#[get("/posts")]
fn get_posts(conn: MyDatabase) -> models::Post {
    models::posts::filter().load(&*conn)
}

fn main() {
    rocket::ignite()
        .attach(fairing::RequestTimer)
        .attach(MyDatabase::fairing())
        .mount("/", routes![
            handler::index, 
            handler::json, 
            handler::big_json, 
            handler::time_now,
            get_posts,
        ])
        .register(
            catchers![
                error::not_found,
                error::internal_server_error],
        )
        .launch();
}

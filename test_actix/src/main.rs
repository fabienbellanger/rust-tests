use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use serde::Serialize;

#[derive(Serialize)]
struct Status {
    status: &'static str,
    message: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!\n")
}

async fn status_json() -> Result<web::Json<Status>, ()> {
    Ok(web::Json(Status {
        status: "OK",
        message: "Success".to_owned(),
    }))
}

#[derive(Serialize)]
pub struct Task {
    id: u32,
    name: &'static str,
    message: String,
}

async fn big_json() -> Result<web::Json<Vec<Task>>, ()> {
    let mut v: Vec<Task> = Vec::new();
    for i in 0..10_000 {
        v.push(Task{
            id: i, 
            name: "Coucou ceci est mon nom", 
            message: String::from("Mon message doit être un peu long pour augmenter la taille"),
        });
    }
    Ok(web::Json(v))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%t | %s | %r | %a | %Ts"))
            .route("/", web::get().to(index))
            .route("/json", web::get().to(status_json))
            .route("/big-json", web::get().to(big_json))
        })
    .bind("localhost:8088")?
    .run()
    .await
}

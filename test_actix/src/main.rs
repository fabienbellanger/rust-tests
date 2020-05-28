use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;

mod models;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!\n")
}

async fn status_json() -> Result<web::Json<models::Status>, ()> {
    Ok(web::Json(models::Status {
        status: "OK",
        message: "Success".to_owned(),
    }))
}

async fn big_json() -> Result<web::Json<Vec<models::Task>>, ()> {
    let mut v: Vec<models::Task> = Vec::new();
    for i in 0..10_000 {
        v.push(models::Task{
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

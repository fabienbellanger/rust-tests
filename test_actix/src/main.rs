use actix_web::{web, App, HttpResponse, HttpServer, Responder};
//use actix_web::middleware::Logger;
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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            //.wrap(Logger::new("%t | %s | %r | %a | %Ts"))
            .route("/", web::get().to(index))
            .route("/json", web::get().to(status_json))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

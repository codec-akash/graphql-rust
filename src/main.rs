use std::io;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("localhost:8080")?
        .run()
        .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

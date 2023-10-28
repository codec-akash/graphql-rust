use std::io;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use actix_web_lab::respond::Html;
use juniper::graphiql::graphiql_source;

mod graphql_schema;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(extractor_path)
            .service(graphql_playground)
    })
    .bind("localhost:8080")?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/users/{user_id}/{friend}")]
async fn extractor_path(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {} , user_id {}", friend, user_id))
}

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql"))
}

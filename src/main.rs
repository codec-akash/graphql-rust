use std::io;

use actix_cors::Cors;
use actix_web::{get, middleware, route, web, App, HttpResponse, HttpServer, Responder, Result};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

mod graphql_schema;
mod routes;
use crate::routes::ping::rping;
mod schemas;
use crate::schemas::schema::{create_schema, Schema};

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let schema = std::sync::Arc::new(create_schema());

    log::info!("Starting on Port: http://localhost:8080/");
    log::info!("Playground running on http://localhost:8080/graphiql");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(schema.clone()))
            // .route("/", web::get().to(index))
            .service(extractor_path)
            .service(graphql_playground)
            .service(rping)
            .service(graphql)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
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
    Html(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}

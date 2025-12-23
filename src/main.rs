use crate::templates::*;
use actix_files::Files;
use actix_web::{App, HttpServer, Responder, get, web};
use db::init_db;

mod db;
mod routes;
mod templates;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = init_db("postgres://ecowan1@localhost:5432/postgres").await;

    println!("🚀 Attempting to start server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
            .service(web::scope("/user").configure(routes::user::services))
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    Index {
        name: "Emmett",
        items: vec!["test", "test", "test", "test", "test", "test"],
    }
}

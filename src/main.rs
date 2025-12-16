use crate::templates::*;
use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use askama::Template;

mod routes;
mod templates;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Attempting to start server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(web::scope("/user").configure(routes::user::services))
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    let template = HelloTemplate {
        name: "Emmett",
        items: vec!["test", "test", "test", "test", "test", "test"],
    };
    HttpResponse::Ok().body(template.render().unwrap())
}

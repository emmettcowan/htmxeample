use crate::templates::HelloTemplate;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use askama::Template;

mod templates;

async fn greet() -> impl Responder {
    let template = HelloTemplate {
        name: "Emmett",
        items: vec!["test", "test", "test", "test", "test", "test"],
    };
    HttpResponse::Ok().body(template.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(greet)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

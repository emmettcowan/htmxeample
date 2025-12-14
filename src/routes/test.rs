use crate::templates::*;
use actix_web::{HttpResponse, Responder, web};
use askama::Template;

#[actix_web::get("/greeting")]
async fn greet() -> impl Responder {
    let template = HelloTemplate {
        name: "Emmett",
        items: vec!["test", "test", "test", "test", "test", "test"],
    };
    HttpResponse::Ok().body(template.render().unwrap())
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(greet);
}

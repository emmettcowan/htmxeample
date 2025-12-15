use crate::templates::*;
use actix_web::{HttpResponse, Responder, web};
use askama::Template;

#[actix_web::get("/userPage")]
async fn profilePage() -> impl Responder {
    let template = ProfileTemplate {};
    HttpResponse::Ok().body(template.render().unwrap())
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(profilePage);
}

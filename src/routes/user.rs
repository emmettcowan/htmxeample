use crate::templates::*;
use actix_web::{Responder, web};

#[actix_web::get("/userPage")]
async fn profile_page() -> impl Responder {
    ProfileTemplate {}
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(profile_page);
}

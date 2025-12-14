use actix_files::Files;
use actix_web::{App, HttpServer, middleware::Logger, web}; // Add middleware::Logger

mod routes;
mod templates;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Attempting to start server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/test").configure(routes::test::services))
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").configure(routes::config))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

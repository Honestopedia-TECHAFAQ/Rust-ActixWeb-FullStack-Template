use actix_web::web;

use crate::handlers::{user_handlers};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/register", web::post().to(user_handlers::register))
    );
}

use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
struct RegisterRequest {
}

#[derive(serde::Serialize)]
struct RegisterResponse {
}

pub async fn register(payload: web::Json<RegisterRequest>) -> HttpResponse {
    HttpResponse::Ok().json(RegisterResponse {})
}


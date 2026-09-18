use crate::utils::api_response;
use actix_web::{Responder, get, web};

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    format!("This is the health check route")
}

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hello, {name}! "))
}

#[get("/test")]
pub async fn test() -> impl Responder {
    api_response::ApiResponse::new(200, "This is a test route".to_string())
}

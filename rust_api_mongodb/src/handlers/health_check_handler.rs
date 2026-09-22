use actix_web::{HttpResponse, Responder, get, web::Data};

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Hello server up")
}

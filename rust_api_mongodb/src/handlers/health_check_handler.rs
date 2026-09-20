use actix_web::{HttpResponse, Responder, get, web::Data};

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Hello server up")
}

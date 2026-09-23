use actix_web::{HttpResponse, Responder, get, web::Data};

/// Get client counts for the current tenant
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Health check", body = String)
    )
)]
#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Hello server up")
}

use actix_web::{
    HttpResponse, get, post, put,
    web::{Data, Json, Path},
};

use crate::models::booking_model::{BookingRequest, FullBooking};
use crate::services::booking_service::BookingService;
use crate::shared::app_state::AppState;

#[utoipa::path(
    post,
    path = "/booking",
    tag = "Bookings",
    security(("bearer_auth" = [])),
    request_body = BookingRequest,
    responses(
        (status = 201, description = "Client created", body = BookingRequest),
        (status = 400, description = "Validation error"),
    )
)]
#[post("/booking")]
pub async fn create_booking(state: Data<AppState>, request: Json<BookingRequest>) -> HttpResponse {
    let service = build_service(&state);
    match service.create_booking(request).await {
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

/// Get client counts for the current tenant
#[utoipa::path(
    get,
    path = "/bookings",
    tag = "Bookings",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Shows the list of bookings", body = Vec<FullBooking>)
    )
)]
#[get("/bookings")]
pub async fn get_bookings(state: Data<AppState>) -> HttpResponse {
    let service = build_service(&state);
    service.get_bookings().await
}

#[utoipa::path(
    put,
    path = "/booking/{id}/cancel",
    tag = "Bookings",
    security(("bearer_auth" = [])),
    request_body = CreateClientRequest,
    responses(
        (status = 201, description = "Booking cancelled", body = ClientResponse),
        (status = 400, description = "Bad Request"),
      
    )
)]
#[put("/booking/{id}/cancel")]
pub async fn cancel_booking(state: Data<AppState>, path: Path<(String,)>) -> HttpResponse {
    let id = path.into_inner().0;
    let service = build_service(&state);
    service.cancel_booking(id).await
}

fn build_service(state: &Data<AppState>) -> BookingService {
    BookingService::new(state.booking_repository.clone())
}

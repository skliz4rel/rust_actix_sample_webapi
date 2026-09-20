use actix_web::{
    HttpResponse, get, post, put,
    web::{Data, Json, Path},
};

use crate::models::booking_model::BookingRequest;
use crate::services::booking_service::BookingService;
use crate::shared::app_state::AppState;

#[post("/booking")]
pub async fn create_booking(state: Data<AppState>, request: Json<BookingRequest>) -> HttpResponse {
    let service = build_service(&state);
    match service.create_booking(request).await {
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("/bookings")]
pub async fn get_bookings(state: Data<AppState>) -> HttpResponse {
    let service = build_service(&state);
    service.get_bookings().await
}

#[put("/booking/{id}/cancel")]
pub async fn cancel_booking(state: Data<AppState>, path: Path<(String,)>) -> HttpResponse {
    let id = path.into_inner().0;
    let service = build_service(&state);
    service.cancel_booking(id).await
}

fn build_service(state: &Data<AppState>) -> BookingService {
    BookingService::new(state.booking_repository.clone())
}

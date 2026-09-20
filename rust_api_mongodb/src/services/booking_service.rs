use crate::models::booking_model::{Booking, BookingRequest};
use crate::repositories::db_traits::booking_db_operation::BookingDbOperation;
use actix_web::HttpResponse;
use actix_web::web::Json;
use mongodb::{error::Error, results::InsertOneResult};
use std::sync::Arc;

pub struct BookingService {
    booking_repository: Arc<dyn BookingDbOperation>,
}

impl BookingService {
    pub fn new(booking_repository: Arc<dyn BookingDbOperation>) -> Self {
        Self { booking_repository }
    }

    pub async fn create_booking(
        &self,
        request: Json<BookingRequest>,
    ) -> Result<InsertOneResult, Error> {
        let booking = Booking::try_from(BookingRequest {
            owner: request.owner.clone(),
            duration_in_minutes: request.duration_in_minutes,
            start_time: request.start_time.clone(),
        })
        .expect("There was an error reading the booking request");

        self.booking_repository.create_booking(booking).await
    }

    pub async fn get_bookings(&self) -> HttpResponse {
        match self.booking_repository.get_bookings().await {
            Ok(booking) => HttpResponse::Ok().json(booking),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

    pub async fn cancel_booking(&self, id: String) -> HttpResponse {
        match self.booking_repository.cancel_booking(id.as_str()).await {
            Ok(booking) => HttpResponse::Ok().json(booking),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }
}

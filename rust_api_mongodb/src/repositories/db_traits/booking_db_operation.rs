use async_trait::async_trait;

use mongodb::{
    error::Error,
    results::{InsertOneResult, UpdateResult},
};

use crate::models::booking_model::{Booking, FullBooking};

#[async_trait]
pub trait BookingDbOperation: Send + Sync {
    async fn create_booking(&self, booking: Booking) -> Result<InsertOneResult, Error>;
    async fn get_bookings(&self) -> Result<Vec<FullBooking>, Error>;
    async fn cancel_booking(&self, booking_id: &str) -> Result<UpdateResult, Error>;
    //
}

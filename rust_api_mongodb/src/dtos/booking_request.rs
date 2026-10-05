use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BookingRequest {
    pub owner: String,
    pub start_time: String,
    pub duration_in_minutes: u8,
}

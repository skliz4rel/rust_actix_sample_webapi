use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OwnerRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

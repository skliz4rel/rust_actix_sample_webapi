use crate::models::owner_model::Owner;
use async_trait::async_trait;
use mongodb::{
    error::Error,
    results::{InsertOneResult, UpdateResult},
};

#[async_trait]
pub trait OwnerDbOperation: Send + Sync {
    async fn create_owner(&self, owner: Owner) -> Result<InsertOneResult, Error>;
}

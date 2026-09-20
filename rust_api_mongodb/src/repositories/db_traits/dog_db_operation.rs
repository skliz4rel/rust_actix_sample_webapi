use crate::models::dog_model::Dog;
use async_trait::async_trait;
use mongodb::{
    error::Error,
    results::{InsertOneResult, UpdateResult},
};

#[async_trait]
pub trait DogDbOperation: Send + Sync {
    async fn create_dog(&self, dog: Dog) -> Result<InsertOneResult, Error>;
}

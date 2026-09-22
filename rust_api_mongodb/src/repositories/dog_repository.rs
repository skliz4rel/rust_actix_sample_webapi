use super::db_traits::dog_db_operation::DogDbOperation;
use crate::config::database::Database;
use crate::models::dog_model::Dog;
use async_trait::async_trait;
use mongodb::{
    error::Error,
    results::{InsertOneResult, UpdateResult},
};
use std::sync::Arc;

pub struct DogRepository {
    database: Arc<Database>,
}

impl DogRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[async_trait]

impl DogDbOperation for DogRepository {
    async fn create_dog(&self, dog: Dog) -> Result<InsertOneResult, Error> {
        let result = self
            .database
            .dog
            .insert_one(dog)
            .await
            .ok()
            .expect("Error creating dog");

        Ok(result)
    }
}

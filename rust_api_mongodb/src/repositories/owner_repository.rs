use super::db_traits::owner_db_operation::OwnerDbOperation;
use crate::config::database::Database;
use crate::models::owner_model::Owner;
use async_trait::async_trait;
use mongodb::{
    error::Error,
    results::{InsertOneResult, UpdateResult},
};
use std::sync::Arc;

pub struct OwnerRepository {
    database: Arc<Database>,
}

impl OwnerRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[async_trait]
impl OwnerDbOperation for OwnerRepository {
    async fn create_owner(&self, owner: Owner) -> Result<InsertOneResult, Error> {
        let result = self
            .database
            .owner
            .insert_one(owner)
            .await
            .ok()
            .expect("Error creating owner");

        Ok(result)
    }
}

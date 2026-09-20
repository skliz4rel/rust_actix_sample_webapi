use super::db_traits::owner_db_operation::OwnerDbOperation;
use crate::config::database::Database;
use crate::models::owner_model::{Owner, OwnerRequest};
use async_trait::async_trait;
use mongodb::{
    error::Error,
    results::{InsertOneResult, UpdateResult},
};
use std::convert::TryFrom;

pub struct OwnerRepository {
    database: Database,
}

impl OwnerRepository {
    pub fn new(database: Database) -> Self {
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

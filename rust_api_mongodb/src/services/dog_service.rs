use crate::models::dog_model::{Dog, DogRequest};
use crate::repositories::db_traits::dog_db_operation::DogDbOperation;
use actix_web::web::Json;
use mongodb::{error::Error, results::InsertOneResult};
use std::sync::Arc;

pub struct DogService {
    dog_repository: Arc<dyn DogDbOperation>,
}

impl DogService {
    pub fn new(dog_repository: Arc<dyn DogDbOperation>) -> Self {
        Self { dog_repository }
    }

    pub async fn create_dog(&self, request: Json<DogRequest>) -> Result<InsertOneResult, Error> {
        let dog = Dog::try_from(DogRequest {
            owner: request.owner.clone(),
            name: request.name.clone(),
            age: request.age,
            breed: request.breed.clone(),
        })
        .expect("Error converting DogRequest to Dog");

        self.dog_repository.create_dog(dog).await
    }
}

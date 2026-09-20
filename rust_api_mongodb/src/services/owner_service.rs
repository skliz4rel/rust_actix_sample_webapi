use crate::models::owner_model::{Owner, OwnerRequest};
use crate::repositories::db_traits::owner_db_operation::OwnerDbOperation;
use actix_web::web::Json;
use mongodb::{error::Error, results::InsertOneResult};
use std::sync::Arc;

pub struct OwnerService {
    owner_repository: Arc<dyn OwnerDbOperation>,
}

impl OwnerService {
    pub fn new(owner_repository: Arc<dyn OwnerDbOperation>) -> Self {
        Self { owner_repository }
    }

    pub async fn create_owner(
        &self,
        request: Json<OwnerRequest>,
    ) -> Result<InsertOneResult, Error> {
        let owner = Owner::try_from(OwnerRequest {
            email: request.email.clone(),
            name: request.name.clone(),
            phone: request.phone.clone(),
            address: request.address.clone(),
        })
        .expect("Error converting OwnerRequest to Owner");

        self.owner_repository.create_owner(owner).await
    }
}

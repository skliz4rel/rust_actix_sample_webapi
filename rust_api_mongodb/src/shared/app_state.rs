use crate::config::database::Database;
use crate::repositories::booking_repository::BookingRepository;
use crate::repositories::db_traits::booking_db_operation::BookingDbOperation;
use crate::repositories::db_traits::dog_db_operation::DogDbOperation;
use crate::repositories::db_traits::owner_db_operation::OwnerDbOperation;
use crate::repositories::dog_repository::DogRepository;
use crate::repositories::owner_repository::OwnerRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub booking_repository: Arc<dyn BookingDbOperation>,
    pub dog_repository: Arc<dyn DogDbOperation>,
    pub owner_repository: Arc<dyn OwnerDbOperation>,
}

impl AppState {
    pub fn new(database: Database) -> Self {
        let database = Arc::new(database);

        Self {
            booking_repository: Arc::new(BookingRepository::new(Arc::clone(&database))),
            dog_repository: Arc::new(DogRepository::new(Arc::clone(&database))),
            owner_repository: Arc::new(OwnerRepository::new(database)),
        }
    }
}

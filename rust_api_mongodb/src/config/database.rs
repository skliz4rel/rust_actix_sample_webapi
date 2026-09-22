use mongodb::{Client, Collection, options::ClientOptions};
use std::env;

use crate::models::{booking_model::Booking, dog_model::Dog, owner_model::Owner};

#[derive(Clone)]
pub struct Database {
    pub booking: mongodb::Collection<Booking>,
    pub dog: mongodb::Collection<Dog>,
    pub owner: mongodb::Collection<Owner>,
}

impl Database {
    pub async fn init() -> Self {
        let _ = dotenvy::dotenv(); //This is a declaration to simple load the .env file from the root part of folder directory

        let uri = env::var("MONGO_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017/?directConnection=true".to_string());
        let database = env::var("DATABASE").unwrap_or_else(|_| "dog_walking".to_string());
        let min_pool = env_u32("MONGO_MIN_POOL", 0);
        let max_pool = env_u32("MONGO_MAX_POOL", 100);

        if max_pool == 0 {
            panic!("MONGO_MAX_POOL must be greater than 0");
        }
        if min_pool > max_pool {
            panic!(
                "MONGO_MIN_POOL ({min_pool}) cannot be greater than MONGO_MAX_POOL ({max_pool})"
            );
        }

        let mut client_options = ClientOptions::parse(&uri)
            .await
            .expect("Failed to parse MongoDB URI");
        client_options.min_pool_size = Some(min_pool);
        client_options.max_pool_size = Some(max_pool);

        let client = Client::with_options(client_options).expect("Failed to create MongoDB client");
        let db = client.database(&database);

        let booking: Collection<Booking> = db.collection("booking");
        let dog: Collection<Dog> = db.collection("dog");
        let owner: Collection<Owner> = db.collection("owner");

        Database {
            booking,
            dog,
            owner,
        }
    }
}

fn env_u32(key: &str, default: u32) -> u32 {
    match env::var(key) {
        Ok(value) => value
            .parse()
            .unwrap_or_else(|_| panic!("{key} must be a non-negative integer, got '{value}'")),
        Err(_) => default,
    }
}

use futures_util::{StreamExt, TryStreamExt};
use mongodb::{
    Client, Collection,
    bson::{doc, from_document, oid::ObjectId},
    error::Error,
    results::{InsertOneResult, UpdateResult},
};
use serde::{Deserialize, Serialize};

use crate::models::{
    booking_model::{Booking, FullBooking},
    dog_model::Dog,
    owner_model::Owner,
};
use std::env;

#[derive(Clone)]
pub struct Database {
    pub booking: mongodb::Collection<Booking>,
    pub dog: mongodb::Collection<Dog>,
    pub owner: mongodb::Collection<Owner>,
}

impl Database {
    pub async fn init() -> Self {
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => "mongodb://localhost:27017/?directConnection=true".to_string(),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("dog_walking");

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

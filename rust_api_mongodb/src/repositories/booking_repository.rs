use super::db_traits::booking_db_operation::BookingDbOperation;
use crate::config::database::Database;
use crate::models::booking_model::{Booking, BookingRequest, FullBooking};
use async_trait::async_trait;
use futures_util::{StreamExt, TryStreamExt};
use mongodb::{
    Client, Collection,
    bson::{doc, from_document, oid::ObjectId},
    error::Error,
    results::{InsertOneResult, UpdateResult},
};

pub struct BookingRepository {
    database: Database,
}

impl BookingRepository {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

#[async_trait]
impl BookingDbOperation for BookingRepository {
    async fn create_booking(&self, booking: Booking) -> Result<InsertOneResult, Error> {
        let result = self
            .database
            .booking
            .insert_one(booking)
            .await
            .ok()
            .expect("Error creating booking");

        Ok(result)
    }

    async fn get_bookings(&self) -> Result<Vec<FullBooking>, Error> {
        let mut results = self
            .database
            .booking
            .aggregate(vec![
                doc! {
                "$match":{
                "cancelled":{"$ne":true}
                }
                },
                doc! {
                "$lookup" : doc! {
                    "from" : "owner",
                    "localField" : "owner",
                    "foreignField" : "_id",
                    "as" : "owner"
                }
                },
                doc! {
                   "$unwind":doc! {
                          "path": "$owner"
                   }
                },
                doc! {
                "$lookup" : doc! {
                    "from" : "dog",
                    "localField" : "owner._id",
                    "foreignField" : "owner",
                    "as" : "dogs"
                }

                },
            ])
            .await?;

        let mut bookings: Vec<FullBooking> = Vec::new();

        while let Some(result) = results.next().await {
            match result {
                Ok(doc) => {
                    let booking: FullBooking =
                        from_document(doc).expect("Error converting document to full booking");
                    bookings.push(booking);
                }
                Err(error) => panic!("Error getting the full booking"),
            }
        }

        Ok(bookings)
    }

    async fn cancel_booking(&self, booking_id: &str) -> Result<UpdateResult, Error> {
        let result = self
            .database
            .booking
            .update_one(
                doc! {
                "_id":ObjectId::parse_str(booking_id).expect("Failed to parse booking_id")
                },
                doc! {
                "$set": doc!{
                       "cancelled":true
                    }
                },
            )
            .await
            .ok()
            .expect("Error cancelling booking");

        Ok(result)
    }
}

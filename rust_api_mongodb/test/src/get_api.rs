use actix_web::{App, http::StatusCode, test, web};
use async_trait::async_trait;
use mongodb::{
    bson::{DateTime, oid::ObjectId},
    error::Error,
    results::{InsertOneResult, UpdateResult},
};
use rust_api_mongodb::models::{
    booking_model::{Booking, FullBooking},
    dog_model::Dog,
    owner_model::Owner,
};
use rust_api_mongodb::repositories::db_traits::{
    booking_db_operation::BookingDbOperation, dog_db_operation::DogDbOperation,
    owner_db_operation::OwnerDbOperation,
};
use rust_api_mongodb::routes;
use rust_api_mongodb::shared::app_state::AppState;
use std::sync::Arc;

struct FakeBookingRepository {
    bookings: Vec<FullBooking>,
    fail: bool,
}

struct StubDogRepository;
struct StubOwnerRepository;

#[async_trait]
impl BookingDbOperation for FakeBookingRepository {
    async fn create_booking(&self, _booking: Booking) -> Result<InsertOneResult, Error> {
        unimplemented!("not used by GET /v1/bookings")
    }

    async fn get_bookings(&self) -> Result<Vec<FullBooking>, Error> {
        if self.fail {
            return Err(std::io::Error::other("failed to load bookings").into());
        }
        Ok(self.bookings.clone())
    }

    async fn cancel_booking(&self, _booking_id: &str) -> Result<UpdateResult, Error> {
        unimplemented!("not used by GET /v1/bookings")
    }
}

#[async_trait]
impl DogDbOperation for StubDogRepository {
    async fn create_dog(&self, _dog: Dog) -> Result<InsertOneResult, Error> {
        unimplemented!("not used by GET routes")
    }
}

#[async_trait]
impl OwnerDbOperation for StubOwnerRepository {
    async fn create_owner(&self, _owner: Owner) -> Result<InsertOneResult, Error> {
        unimplemented!("not used by GET routes")
    }
}

fn sample_booking() -> FullBooking {
    let owner_id = ObjectId::new();
    FullBooking {
        _id: ObjectId::new(),
        owner: Owner {
            _id: owner_id,
            name: "Ada Lovelace".to_string(),
            email: "ada@example.com".to_string(),
            phone: "555-0100".to_string(),
            address: "12 Analytical Engine Rd".to_string(),
        },
        dogs: vec![Dog {
            _id: ObjectId::new(),
            owner: owner_id,
            name: Some("Nala".to_string()),
            age: Some(4),
            breed: Some("Labrador".to_string()),
        }],
        start_time: DateTime::from_millis(1_726_848_000_000),
        duration_in_minutes: 30,
        cancelled: false,
    }
}

fn test_state(bookings: Vec<FullBooking>, fail: bool) -> AppState {
    AppState {
        booking_repository: Arc::new(FakeBookingRepository { bookings, fail }),
        dog_repository: Arc::new(StubDogRepository),
        owner_repository: Arc::new(StubOwnerRepository),
    }
}

async fn get(state: AppState, uri: &str) -> (StatusCode, web::Bytes) {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(routes::config),
    )
    .await;
    let req = test::TestRequest::get().uri(uri).to_request();
    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    (status, body)
}

#[actix_web::test]
async fn get_health_returns_ok() {
    let (status, body) = get(test_state(Vec::new(), false), "/v1/health").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "Hello server up");
}

#[actix_web::test]
async fn get_bookings_returns_empty_list() {
    let (status, body) = get(test_state(Vec::new(), false), "/v1/bookings").await;

    assert_eq!(status, StatusCode::OK);
    let json: serde_json::Value = serde_json::from_slice(&body).expect("JSON body");
    assert_eq!(json, serde_json::json!([]));
}

#[actix_web::test]
async fn get_bookings_returns_joined_booking() {
    let booking = sample_booking();
    let (status, body) = get(test_state(vec![booking], false), "/v1/bookings").await;

    assert_eq!(status, StatusCode::OK);
    let json: serde_json::Value = serde_json::from_slice(&body).expect("JSON body");
    assert_eq!(json.as_array().map(|rows| rows.len()), Some(1));
    assert_eq!(json[0]["owner"]["name"], "Ada Lovelace");
    assert_eq!(json[0]["dogs"][0]["name"], "Nala");
    assert_eq!(json[0]["duration_in_minutes"], 30);
    assert_eq!(json[0]["cancelled"], false);
}

#[actix_web::test]
async fn get_bookings_returns_500_when_repository_fails() {
    let (status, _body) = get(test_state(Vec::new(), true), "/v1/bookings").await;

    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
}

#[actix_web::test]
async fn get_unknown_route_returns_404() {
    let (status, _body) = get(test_state(Vec::new(), false), "/v1/unknown").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

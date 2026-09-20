use actix_web::{
    HttpResponse, post,
    web::{Data, Json},
};

use crate::models::dog_model::DogRequest;
use crate::services::dog_service::DogService;
use crate::shared::app_state::AppState;

#[post("/dog")]
pub async fn create_dog(state: Data<AppState>, request: Json<DogRequest>) -> HttpResponse {
    let service = build_service(&state);
    match service.create_dog(request).await {
        Ok(dog) => HttpResponse::Ok().json(dog),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

fn build_service(state: &Data<AppState>) -> DogService {
    DogService::new(state.dog_repository.clone())
}

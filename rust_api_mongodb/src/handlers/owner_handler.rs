use actix_web::{
    HttpResponse, post,
    web::{Data, Json},
};

use crate::models::owner_model::OwnerRequest;
use crate::services::owner_service::OwnerService;
use crate::shared::app_state::AppState;

#[utoipa::path(
    post,
    path = "/owner",
    tag = "Owners",
    security(("bearer_auth" = [])),
    request_body = OwnerRequest,
    responses(
        (status = 201, description = "Owner created", body = OwnerRequest),
        (status = 400, description = "Request Validation error"),
       
    )
)]
#[post("/owner")]
pub async fn create_owner(state: Data<AppState>, request: Json<OwnerRequest>) -> HttpResponse {
    let service = build_service(&state);
    match service.create_owner(request).await {
        Ok(owner) => HttpResponse::Ok().json(owner),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

fn build_service(state: &Data<AppState>) -> OwnerService {
    OwnerService::new(state.owner_repository.clone())
}

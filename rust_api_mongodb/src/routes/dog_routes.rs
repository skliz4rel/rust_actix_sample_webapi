use actix_web::web;

use crate::handlers::dog_handler::create_dog;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1").service(create_dog));
}

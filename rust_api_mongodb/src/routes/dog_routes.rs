use actix_web::web;

use crate::handlers::dog_handler::create_dog;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_dog);
}

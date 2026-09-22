use actix_web::web;

use crate::handlers::owner_handler::create_owner;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_owner);
}

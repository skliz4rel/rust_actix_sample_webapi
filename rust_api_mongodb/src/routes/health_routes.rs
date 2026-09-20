use actix_web::web;

use crate::handlers::health_check_handler::health_check;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}

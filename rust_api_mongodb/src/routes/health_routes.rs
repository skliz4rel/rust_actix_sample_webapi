use actix_web::web;

use crate::handlers::health_check_handler::health_check;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1").service(health_check));
}

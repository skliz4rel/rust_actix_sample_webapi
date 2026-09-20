use actix_web::web;

use crate::handlers::booking_handler::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(create_booking)
            .service(get_bookings)
            .service(cancel_booking),
    );
}

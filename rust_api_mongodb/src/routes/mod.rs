use actix_web::web;

pub mod booking_routes;
pub mod dog_routes;
pub mod health_routes;
pub mod owner_routes;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .configure(health_routes::config)
            .configure(booking_routes::config)
            .configure(dog_routes::config)
            .configure(owner_routes::config),
    );
}

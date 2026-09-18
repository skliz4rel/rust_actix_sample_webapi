use actix_web::web;

use super::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/home")
                .service(handlers::home_handler::greet)
                .service(handlers::home_handler::test), //This routes within this scope would be prefixed with /home
        )
        .service(handlers::home_handler::health_check); //This route is not within the scope, so it would be accessible at /health_check
}

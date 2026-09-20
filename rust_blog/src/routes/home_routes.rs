use actix_web::web;

use super::handlers; //super was used because, it a sub folder of the routes folder. The handlers module is in the same folder as the home_routes.rs file, so we can use super to access it.

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/home")
                .service(handlers::home_handler::greet)
                .service(handlers::home_handler::test), //This routes within this scope would be prefixed with /home
        )
        .service(handlers::home_handler::health_check); //This route is not within the scope, so it would be accessible at /health_check
}

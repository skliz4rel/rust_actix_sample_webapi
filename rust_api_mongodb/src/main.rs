//The mod statement below make the package avialable to the main.rs file. The mod statement is used to declare a module in Rust. In this case, it declares the database module, which is defined in the src/database/mod.rs file. This allows the main.rs file to access the contents of the database module, such as the db submodule and any functions or types defined within it.
//It also makes it available to other rust files in the project, allowing them to use the database module's functionality by importing it with a use statement. This is important for organizing code and separating concerns in larger Rust projects.
mod config;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;
mod shared;

use crate::config::database::Database;
use crate::shared::app_state::AppState;
use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await;
    let app_state = AppState::new(db); //contains all the state of the data service for the application

    HttpServer::new(move || {
        App::new()
            //.app_data(db_data.clone())
            .app_data(web::Data::new(app_state.clone()))
            .configure(routes::health_routes::config)
            .configure(routes::booking_routes::config)
            .configure(routes::dog_routes::config)
            .configure(routes::owner_routes::config)
    })
    .bind(("localhost", 5001))?
    .run()
    .await
}

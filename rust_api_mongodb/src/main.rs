use actix_web::{App, HttpServer, web};
use rust_api_mongodb::config::database::Database;
use rust_api_mongodb::routes;
use rust_api_mongodb::shared::app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await;
    let app_state = web::Data::new(AppState::new(db));

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(routes::config)
    })
    .bind(("localhost", 5001))?
    .run()
    .await
}

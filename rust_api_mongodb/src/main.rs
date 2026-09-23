use actix_web::{App, HttpServer, web};
use rust_api_mongodb::config::config::Config;
use rust_api_mongodb::config::database::Database;
use rust_api_mongodb::shared::app_state::AppState;
use rust_api_mongodb::{routes, swagger::api_doc::ApiDoc};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenvy::dotenv();
    // let server: String = env::var("SERVER").unwrap_or_else(|_| "119.0.0.1".to_string());
    // let port: String = env::var("PORT").unwrap_or_else(|_| "100".to_string());

    let config: Config = Config::new()
        .expect("Failed to load configuration, add config to .env file in the root directory");

    let db = Database::init().await;
    let app_state = web::Data::new(AppState::new(db));

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(routes::config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind((config.server, config.port))?
    .run()
    .await
}

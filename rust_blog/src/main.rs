use actix_web::{App, HttpServer, Responder, get, middleware::Logger, web};

mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }

    dotenv::dotenv().ok(); //declare the env variable from the .env file. This will load the variables from the .env file into the environment variables of the application.
    env_logger::init();

    //we are going to call the constants from the utils module
    let address = utils::constants::ADDRESS.clone();
    let port = *utils::constants::PORT;

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) //The wrap is used to add middleware to the application. In this case, we are adding a logger middleware that will log all incoming requests and their responses.
            .configure(routes::home_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}

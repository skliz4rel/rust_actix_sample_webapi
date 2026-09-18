use lazy_static::lazy_static;

lazy_static! {
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
}

fn set_address() -> String {
    dotenv::dotenv().ok();
    dotenv::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1".into())
}

fn set_port() -> u16 {
    dotenv::dotenv().ok();
    dotenv::var("PORT")
        .unwrap_or_else(|_| "8080".into())
        .parse()
        .unwrap_or(8080)
}

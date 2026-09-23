use std::env::{self, VarError};

pub struct Config {
    pub server: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Result<Config, VarError> {
        let config = Self {
            server: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),

            port: env::var("PORT")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .expect("invalid port value was configured"),
        };

        Ok(config)
    }
}

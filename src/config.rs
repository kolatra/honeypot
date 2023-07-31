use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub db_user: String,
    pub db_password: String,
}

impl Config {
    pub async fn read() -> crate::Result<Config> {
        let config = Config {
            port: std::env::var("PORT")?.parse()?,
            db_user: std::env::var("DB_USER")?,
            db_password: std::env::var("DB_PASSWORD")?,
        };

        Ok(config)
    }
}

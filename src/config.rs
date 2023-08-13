use dotenvy::var;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub db_url: String,
}

impl Config {
    pub fn new() -> anyhow::Result<Config> {
        let config = Config {
            port: var("PORT")?.parse()?,
            db_url: var("DATABASE_URL")?,
        };

        dbg!(&config);

        Ok(config)
    }
}

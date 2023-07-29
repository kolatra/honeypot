use std::{fs::File, io::Read};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub db_user: String,
    pub db_password: String,
}

impl Config {
    pub async fn read() -> crate::Result<Config> {
        let mut config_file = File::open("config.toml")?;

        let mut config_contents = String::new();
        config_file.read_to_string(&mut config_contents)?;

        let config_toml: Config = toml::from_str(&config_contents)?;

        let config = Config {
            port: config_toml.port,
            db_user: config_toml.db_user,
            db_password: config_toml.db_password,
        };

        Ok(config)
    }
}

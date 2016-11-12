use std::error::Error;
use ::std::io::prelude::*;
use ::std::fs::File;
use serde_json;

#[derive(Deserialize)]
pub struct AgentConfig {
    pub listen: String,
    pub dbms: Option<DbmsConfig>,
    pub cache: Option<CacheConfig>,
}

impl AgentConfig {
    pub fn read(filename: &str) -> Option<AgentConfig> {
        let mut config_data = String::new();

        let mut config_file = match File::open(&filename) {
            Err(why) => panic!("Couldn't open {}: {}", &filename, why.description()),
            Ok(file) => file,
        };

        match config_file.read_to_string(&mut config_data) {
            Ok(_) => serde_json::from_str(&config_data).unwrap(),
            Err(_) => None,
        }
    }
}

#[derive(Deserialize)]
pub struct DbmsConfig {
    pub relay: Option<String>,
    pub app: Option<String>,
    pub host: Option<String>,
    pub port: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct CacheConfig {
    pub relay: Option<String>,
    pub app: Option<String>,
}

#[derive(Deserialize)]
pub struct ClientConfig {
    pub agent_address: String,
    pub user: User,
    pub crypto: Crypto,
}

impl ClientConfig {
    pub fn read(filename: &str) -> Option<ClientConfig> {
        let mut config_data = String::new();

        let mut config_file = match File::open(&filename) {
            Err(why) => panic!("Couldn't open {}: {}", &filename, why.description()),
            Ok(file) => file,
        };

        match config_file.read_to_string(&mut config_data) {
            Ok(_) => serde_json::from_str(&config_data).unwrap(),
            Err(_) => None,
        }
    }
}

#[derive(Deserialize)]
pub struct User {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Crypto {
    pub pub_key_file: String,
    pub priv_key_file: String,
}

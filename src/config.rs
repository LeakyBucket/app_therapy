use std::error::Error;
use ::std::io::prelude::*;
use ::std::fs::File;
use serde_json;

#[derive(Deserialize)]
pub struct Config {
    pub agent_address: String,
    pub user: User,
    pub crypto: Crypto,
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

pub fn load_config() -> Option<Config> {
    let mut config_data = String::new();

    let mut config_file = match File::open("app_therapy.json") {
        Err(why) => panic!("Couldn't open app_therapy.json: {}", why.description()),
        Ok(file) => file,
    };

    match config_file.read_to_string(&mut config_data) {
        Ok(_) => serde_json::from_str(&config_data).unwrap(),
        Err(_) => None,
    }
}

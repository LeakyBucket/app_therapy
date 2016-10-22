extern crate app_therapy;

use app_therapy::config;

fn main() {
    let config = match config::load_config() {
        Some(config) => config,
        None => panic!("Failed to parse config file!"),
    };

    println!("User: {}\nPassword: {}", config.user.login, config.user.password);
}

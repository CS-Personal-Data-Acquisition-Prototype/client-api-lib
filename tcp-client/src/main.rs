mod api;
mod config;
mod requests;

use api::{auth, sensor, session, session_sensor, session_sensor_data, user};
use config::Config;
use reqwest::Client;
use tokio;

pub fn get_client() -> Client {
    let client = Client::new();

    return client;
}

pub fn get_config() -> Config {
    let config = Config::new();

    return config;
}

async fn main() {
    println!("Starting client...");
}

mod api;
mod path;
mod requests;

use api::{auth, sensor, session, session_sensor, session_sensor_data, user};
use path::Path;
use reqwest::Client;
use tokio;

pub fn get_client() -> Client {
    let client = Client::new();

    return client;
}

pub fn get_path() -> Path {
    let path = Path::new();

    return path;
}

async fn main() {
    println!("Starting client...");
}

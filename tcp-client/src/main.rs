pub mod api;
pub mod path;
pub mod requests;

use api::{auth, sensor, session, session_sensor, session_sensor_data, user};
use reqwest_wasm::Client;

pub fn get_client() -> Client {
    let client = Client::new();

    return client;
}

fn main() {
    println!("Starting client...");
}

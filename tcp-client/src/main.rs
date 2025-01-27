// use argon2::{Argon2, PasswordHasher};
use reqwest::Client;
// use serde::{Deserialize, Serialize};
// use serde_json;
use tokio;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let response = client
        .get("http://127.0.0.1:7878")
        .send()
        .await
        .expect("Failed to send request");

    let body = response.text().await.expect("Failed to read response body");
    println!("Response: {}", body);
}

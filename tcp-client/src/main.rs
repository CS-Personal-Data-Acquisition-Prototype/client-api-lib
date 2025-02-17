mod api;
mod config;
mod requests;

use api::user;
use config::Config;
use reqwest::Client;
use tokio;

#[tokio::main]
async fn main() {
    let config = Config::new();
    let client = Client::new();

    // Following code used to test interface with server
    let username = "username";
    let pw = "working_pw";
    let session_id = "skdjfksjf";

    let mut _res = user::create_user(&client, &config, username, pw).await;
    _res = user::view_all_users(&client, &config).await;
    _res = user::view_user_profile(&client, &config, session_id).await;
}

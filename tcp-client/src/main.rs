use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use reqwest::{
    header::{HeaderName, HeaderValue, CONTENT_TYPE},
    Client, Url,
};
use serde::Serialize;
use serde_json;
use std::error::Error;
use tokio;

// TODO: Find out how to not make a new Client every time
// TODO: Cookie jar implementation and add session ID to HTTP header

#[derive(Debug, Serialize)]
struct UserPassword {
    username: String,
    password_hash: String,
}

// Passwords cannot be same as username or less than ten characters long
fn valid_password(username: &str, password: &str) -> bool {
    if username == password || password.chars().count() < 10 {
        return false;
    } else {
        return true;
    }
}

fn hash_pw(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let argon2id = Argon2::default();

    match argon2id.hash_password(password, &salt) {
        Ok(hashed_pw) => hashed_pw.to_string(),
        Err(e) => panic!("Unable to hash password: {:?}", e),
    }
}

async fn create_user(client: &Client, username: &str, pw: &str) -> Result<(), Box<dyn Error>> {
    if !valid_password(username, pw) {
        // TODO: Should display password issues in GUI
        panic!("Invalid password");
    }

    let hashed = hash_pw(pw.as_bytes());
    let params = UserPassword {
        username: username.to_string(),
        password_hash: hashed,
    };

    // let client = Client::new();
    let res = client
        .post("http://127.0.0.1:7878/users")
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .json(&params)
        .send()
        .await?;

    println!("Response status: {}", res.status());

    let json: serde_json::Value = res.json().await?;
    let json_string = serde_json::to_string_pretty(&json).unwrap();
    println!("{}", json_string);

    Ok(())
}

async fn view_all_users(client: &Client) -> Result<(), Box<dyn Error>> {
    let res = client.get("http://127.0.0.1:7878/users").send().await?;

    println!("Response status: {}", res.status());

    let json: serde_json::Value = res.json().await?;
    let json_string = serde_json::to_string_pretty(&json).unwrap();
    println!("{}", json_string);

    Ok(())
}

// TODO: Should be storing session id somewhere
async fn view_user_profile(client: &Client, session_id: &str) -> Result<(), Box<dyn Error>> {
    let res = client
        .get("http://127.0.0.1:7878/users/profile")
        .header(
            HeaderName::from_static("session_id"),
            HeaderValue::from_str(session_id)?,
        )
        .send()
        .await?;

    println!("Response status: {}", res.status());

    let json: serde_json::Value = res.json().await?;
    let json_string = serde_json::to_string_pretty(&json).unwrap();
    println!("{}", json_string);

    Ok(())
}

async fn view_user_by_username(client: &Client, username: &str) -> Result<(), Box<dyn Error>> {
    let api_base = Url::parse(format!("http://127.0.0.1:7878/users/{username}").as_str())?;
    let res = client.get(api_base).send().await?;

    println!("Response status: {}", res.status());

    let json: serde_json::Value = res.json().await?;
    let json_string = serde_json::to_string_pretty(&json).unwrap();
    println!("{}", json_string);

    Ok(())
}

async fn update_user(client: &Client, username: &str, pw: &str) -> Result<(), Box<dyn Error>> {
    let hashed = hash_pw(pw.as_bytes());
    let params = UserPassword {
        username: username.to_string(),
        password_hash: hashed,
    };

    let api_base = Url::parse(format!("http://127.0.0.1:7878/users/{username}").as_str())?;

    let res = client
        .patch(api_base)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .json(&params)
        .send()
        .await?;

    println!("Response status: {}", res.status());

    let json: serde_json::Value = res.json().await?;
    let json_string = serde_json::to_string_pretty(&json).unwrap();
    println!("{}", json_string);

    Ok(())
}

async fn delete_user(client: &Client, username: &str) -> Result<(), Box<dyn Error>> {
    let api_base = Url::parse(format!("http://127.0.0.1:7878/users/{username}").as_str())?;
    let res = client.delete(api_base).send().await?;

    println!("Response status: {}", res.status());

    let json: serde_json::Value = res.json().await?;
    let json_string = serde_json::to_string_pretty(&json).unwrap();
    println!("{}", json_string);

    Ok(())
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    let username = "username";
    let pw = "working_pw";

    let mut _res = create_user(&client, username, pw).await;
    _res = view_all_users(&client).await;
}

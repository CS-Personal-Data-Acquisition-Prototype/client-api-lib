#![allow(dead_code)]

use crate::config::Config;
use crate::requests::send_request::send_request;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use reqwest::Client;
use reqwest::Method;
use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize)]
pub struct UserPassword {
    pub username: String,
    pub password_hash: String,
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

pub async fn create_user(
    client: &Client,
    config: &Config,
    username: &str,
    pw: &str,
) -> Result<(), Box<dyn Error>> {
    // TODO: Handle this without breaking program
    if !valid_password(username, pw) {
        panic!("Invalid password");
    }

    let hashed = hash_pw(pw.as_bytes());
    let params = UserPassword {
        username: username.to_string(),
        password_hash: hashed,
    };

    let url = &config.get_user_url();

    let (status, json) = send_request(client, &Method::POST, url, None, Some(&params)).await?;

    println!("Response status: {}", status);
    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    Ok(())
}

pub async fn view_all_users(client: &Client, config: &Config) -> Result<(), Box<dyn Error>> {
    let url = &config.get_user_url();

    let (status, json) = send_request(client, &Method::GET, url, None, None::<()>).await?;

    println!("Response status: {}", status);
    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    Ok(())
}

// TODO: Should be storing session id somewhere
pub async fn view_user_profile(
    client: &Client,
    config: &Config,
    session_id: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_profile_url();

    let (status, json) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await?;

    println!("Response status: {}", status);
    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    Ok(())
}

pub async fn view_user_by_username(
    client: &Client,
    config: &Config,
    username: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_username_url(username);

    let (status, json) = send_request(client, &Method::GET, url, None, None::<()>).await?;

    println!("Response status: {}", status);
    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    Ok(())
}

pub async fn update_user(
    client: &Client,
    config: &Config,
    username: &str,
    pw: &str,
) -> Result<(), Box<dyn Error>> {
    let hashed = hash_pw(pw.as_bytes());
    let params = UserPassword {
        username: username.to_string(),
        password_hash: hashed,
    };

    let url = &config.get_username_url(username);

    let (status, json) = send_request(client, &Method::PATCH, url, None, Some(&params)).await?;

    println!("Response status: {}", status);
    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    Ok(())
}

pub async fn delete_user(
    client: &Client,
    config: &Config,
    username: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_username_url(username);

    let (status, json) = send_request(client, &Method::DELETE, url, None, None::<()>).await?;

    println!("Response status: {}", status);
    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    Ok(())
}

#![allow(dead_code)]

use crate::config::Config;
use crate::requests::send_request::send_request;
use reqwest::{Client, Method};
use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
}

pub async fn create_user(
    client: &Client,
    config: &Config,
    username: &str,
    pw: &str,
) -> Result<(), Box<dyn Error>> {
    let params = User {
        username: username.to_string(),
        password_hash: pw.to_string(),
    };

    let url = &config.get_user_url();

    let (status, json, _headers) =
        send_request(client, &Method::POST, url, None, Some(&params)).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    Ok(())
}

pub async fn view_all_users(
    client: &Client,
    config: &Config,
    session_id: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_user_url();

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    Ok(())
}

pub async fn view_user_profile(
    client: &Client,
    config: &Config,
    session_id: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_profile_url();

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    Ok(())
}

pub async fn view_user_by_username(
    client: &Client,
    config: &Config,
    username: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_username_url(username);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, None, None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    Ok(())
}

pub async fn update_user(
    client: &Client,
    config: &Config,
    username: &str,
    pw: &str,
) -> Result<(), Box<dyn Error>> {
    let params = User {
        username: username.to_string(),
        password_hash: pw.to_string(),
    };

    let url = &config.get_username_url(username);

    let (status, json, _headers) =
        send_request(client, &Method::PATCH, url, None, Some(&params)).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    Ok(())
}

pub async fn delete_user(
    client: &Client,
    config: &Config,
    username: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_username_url(username);

    let (status, json, _headers) =
        send_request(client, &Method::DELETE, url, None, None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    Ok(())
}

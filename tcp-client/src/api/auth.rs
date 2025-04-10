#![allow(dead_code)]

use crate::config::Config;
use crate::requests::send_request::send_request;
use reqwest::{header::SET_COOKIE, Client, Method};
use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
}

fn get_session_id(cookie_str: &str) -> Option<String> {
    for part in cookie_str.split(';') {
        if part.starts_with("session_id=") {
            return Some(part["session-id=".len()..].to_string());
        }
    }
    None
}

pub async fn user_login(
    client: &Client,
    config: &Config,
    username: &str,
    pw: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    let url = &config.get_login_url();
    let params = User {
        username: username.to_string(),
        password_hash: pw.to_string(),
    };

    let (status, json, headers) =
        send_request(client, &Method::POST, url, None, Some(&params)).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    if let Some(cookie) = headers.get(SET_COOKIE) {
        let cookie_str = cookie.to_str()?;
        if let Some(session_id) = get_session_id(cookie_str) {
            println!("Session ID: {}", session_id);
            return Ok(Some(session_id));
        }
    }

    Ok(None)
}

pub async fn user_logout(
    client: &Client,
    config: &Config,
    session_id: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    let url = &config.get_logout_url();

    let (status, json, headers) =
        send_request(client, &Method::POST, url, Some(session_id), None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    if let Some(cookie) = headers.get(SET_COOKIE) {
        let cookie_str = cookie.to_str()?;
        if let Some(session_id) = get_session_id(cookie_str) {
            println!("Session ID: {}", session_id);
            return Ok(Some(session_id));
        }
    }

    Ok(None)
}

pub async fn renew_session(
    client: &Client,
    config: &Config,
    session_id: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    let url = &config.get_renew_url();

    let (status, json, headers) =
        send_request(client, &Method::POST, url, Some(session_id), None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    if let Some(cookie) = headers.get(SET_COOKIE) {
        let cookie_str = cookie.to_str()?;
        if let Some(session_id) = get_session_id(cookie_str) {
            println!("Session ID: {}", session_id);
            return Ok(Some(session_id));
        }
    }

    Ok(None)
}

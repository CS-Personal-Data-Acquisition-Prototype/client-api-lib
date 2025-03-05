#![allow(dead_code)]

use crate::config::Config;
use crate::requests::send_request::send_request;
use reqwest::{Client, Method};
use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize)]
pub struct Session {
    pub username: String,
}

pub async fn create_session(
    client: &Client,
    config: &Config,
    session_id: &str,
    username: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_sessions_url();
    let params = Session {
        username: username.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::POST, url, Some(session_id), Some(&params)).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    Ok(())
}

pub async fn view_all_sessions(
    client: &Client,
    config: &Config,
    session_id: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_sessions_url();

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    Ok(())
}

pub async fn view_sessions_by_user(
    client: &Client,
    config: &Config,
    session_id: &str,
    username: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_sessions_subpath_url("user", username);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    Ok(())
}

pub async fn view_session_by_id(
    client: &Client,
    config: &Config,
    session_id: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_sessions_subpath_url("id", session_id);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    Ok(())
}

pub async fn update_session(
    client: &Client,
    config: &Config,
    session_id: &str,
    username: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_sessions_exp_url(session_id);
    let params = Session {
        username: username.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::PATCH, url, Some(session_id), Some(&params)).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    Ok(())
}

pub async fn delete_session(
    client: &Client,
    config: &Config,
    session_id: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_sessions_exp_url(session_id);

    let (status, json, _headers) =
        send_request(client, &Method::DELETE, url, Some(session_id), None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    Ok(())
}

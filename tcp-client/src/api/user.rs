//! Requests for the user endpoint

#![allow(dead_code)]

use crate::path::Path;
use crate::requests::send_request::send_request;
use reqwest::{Client, Method, StatusCode};
use serde_json::Value;
use serde::Serialize;

/// Struct defining a user
#[derive(Debug, Serialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
}

/// Send request to create a new user
pub async fn create_user(
    client: &Client,
    path: &Path,
    username: &str,
    pw: &str,
) -> (StatusCode, Option<Value>) {
    let params = User {
        username: username.to_string(),
        password_hash: pw.to_string(),
    };

    let url = &path.get_user_url();

    let (status, json, _headers) =
        send_request(client, &Method::POST, url, None, Some(&params)).await;

    (status, json)
}

/// Send request to get all users
pub async fn view_all_users(
    client: &Client,
    path: &Path,
    session_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_user_url();

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await;

    (status, json)
}

/// Send request to get user currently loggged in
pub async fn view_user_profile(
    client: &Client,
    path: &Path,
    session_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_profile_url();

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await;

    (status, json)
}

/// Send request to get a specific user by username
pub async fn view_user_by_username(
    client: &Client,
    path: &Path,
    username: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_username_url(username);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, None, None::<()>).await;

    (status, json)
}

/// Send request to partially or fully update a user
pub async fn update_user(
    client: &Client,
    path: &Path,
    username: &str,
    pw: &str,
) -> (StatusCode, Option<Value>) {
    let params = User {
        username: username.to_string(),
        password_hash: pw.to_string(),
    };

    let url = &path.get_username_url(username);

    let (status, json, _headers) =
        send_request(client, &Method::PATCH, url, None, Some(&params)).await;

    (status, json)
}

/// Send request to delete a user by username
pub async fn delete_user(
    client: &Client,
    path: &Path,
    username: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_username_url(username);

    let (status, json, _headers) =
        send_request(client, &Method::DELETE, url, None, None::<()>).await;

    (status, json)
}

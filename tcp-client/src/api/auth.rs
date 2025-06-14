//! Requests for the authentication endpoint

use crate::path::auth;
use crate::requests::send_request::send_request;
use reqwest_wasm::{header::SET_COOKIE, Client, Method, StatusCode};
use serde::Serialize;
use serde_json::Value;

/// Struct defining a user
#[derive(Debug, Serialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
}

/// Helper function to extract session ID from the HTTP header
fn get_session_id(cookie_str: &str) -> Option<String> {
    for part in cookie_str.split(';') {
        if part.starts_with("session_id=") {
            return Some(part["session-id=".len()..].to_string());
        }
    }
    eprintln!("Could not extract session_id");
    None
}

/// Send request to attempt login with provided user credentials
pub async fn user_login(
    client: &Client,
    username: &str,
    pw: &str,
) -> (StatusCode, Option<Value>, Option<String>) {
    let url = auth::get_login_url();
    let params = User {
        username: username.to_string(),
        password_hash: pw.to_string(),
    };

    let (status, json, headers) =
        send_request(client, &Method::POST, url, None, Some(&params)).await;

    // Get the session id from the SET_COOKIE header
    if let Some(cookie) = headers.get(SET_COOKIE).and_then(|cookie| cookie.to_str().ok()) {
        if let Some(new_session_id) = get_session_id(cookie) {
            return (status, json, Some(new_session_id));
        }
    }
    // Return None if new_session_id cannot be extracted
    (status, json, None)
}


/// Send request to log out the current user
pub async fn user_logout(
    client: &Client,
    session_id: &str,
) -> (StatusCode, Option<Value>, String) {
    let url = auth::get_logout_url();

    let (status, json, headers) =
        send_request(client, &Method::POST, url, Some(session_id), None::<()>).await;

    // Get the session id from the SET_COOKIE header
    if let Some(cookie) = headers.get(SET_COOKIE).and_then(|cookie| cookie.to_str().ok()) {
        if let Some(new_session_id) = get_session_id(cookie) {
            return (status, json, new_session_id);
        }
    }
    // Return old session_id if new_session_id cannot be extracted
    (status, json, session_id.to_string())
}


/// Send request to renew session tokens
pub async fn renew_session(
    client: &Client,
    session_id: &str,
) -> (StatusCode, Option<Value>, String) {
    let url = auth::get_renew_url();

    let (status, json, headers) =
        send_request(client, &Method::POST, url, Some(session_id), None::<()>).await;

    // Get the session id from the SET_COOKIE header
    if let Some(cookie) = headers.get(SET_COOKIE).and_then(|cookie| cookie.to_str().ok()) {
        if let Some(new_session_id) = get_session_id(cookie) {
            return (status, json, new_session_id);
        }
    }
    // Return old session_id if new_session_id cannot be extracted
    (status, json, session_id.to_string())
}

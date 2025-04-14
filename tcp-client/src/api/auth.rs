#![allow(dead_code)]

use crate::path::Path;
use crate::requests::send_request::send_request;
use reqwest::{header::SET_COOKIE, Client, Method, StatusCode};
use serde::Serialize;
use serde_json::Value;

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
    eprintln!("Could not extract session_id");
    None
}

pub async fn user_login(
    client: &Client,
    path: &Path,
    username: &str,
    pw: &str,
) -> (StatusCode, Option<Value>, Option<String>) {
    let url = &path.get_login_url();
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

pub async fn user_logout(
    client: &Client,
    path: &Path,
    session_id: &str,
) -> (StatusCode, Option<Value>, String) {
    let url = &path.get_logout_url();

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

pub async fn renew_session(
    client: &Client,
    path: &Path,
    session_id: &str,
) -> (StatusCode, Option<Value>, String) {
    let url = &path.get_renew_url();

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

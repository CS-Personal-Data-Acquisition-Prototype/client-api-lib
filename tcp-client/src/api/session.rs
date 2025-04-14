#![allow(dead_code)]

use crate::path::Path;
use crate::requests::send_request::send_request;
use reqwest::{Client, Method, StatusCode};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct Session {
    pub username: String,
}

pub async fn create_session(
    client: &Client,
    path: &Path,
    session_id: &str,
    username: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_sessions_url();
    let params = Session {
        username: username.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::POST, url, Some(session_id), Some(&params)).await;

    (status, json)
}

pub async fn view_all_sessions(
    client: &Client,
    path: &Path,
    session_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_sessions_url();

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await;

    (status, json)
}

pub async fn view_sessions_by_user(
    client: &Client,
    path: &Path,
    session_id: &str,
    username: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_sessions_subpath_url("user", username);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await;

    (status, json)
}

pub async fn view_session_by_id(
    client: &Client,
    path: &Path,
    session_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_sessions_subpath_url("id", session_id);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await;

    (status, json)
}

pub async fn update_session(
    client: &Client,
    path: &Path,
    session_id: &str,
    username: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_sessions_exp_url(session_id);
    let params = Session {
        username: username.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::PATCH, url, Some(session_id), Some(&params)).await;

    (status, json)
}

pub async fn delete_session(
    client: &Client,
    path: &Path,
    session_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_sessions_exp_url(session_id);

    let (status, json, _headers) =
        send_request(client, &Method::DELETE, url, Some(session_id), None::<()>).await;

    (status, json)
}

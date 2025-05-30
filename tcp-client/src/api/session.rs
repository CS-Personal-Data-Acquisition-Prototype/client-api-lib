//! Requests for the session endpoint

//session id disabled for temporary solution

use crate::path::session;
use crate::requests::send_request::send_request;
use reqwest_wasm::{Client, Method, StatusCode};
use serde::Serialize;
use serde_json::Value;

/// Struct defining a session
#[derive(Debug, Serialize)]
pub struct Session {
    //pub id: String,
    pub username: String,
}

/// Struct defining a session for PATCH
// #[derive(Debug, Serialize)]
// pub struct PatchSession {
//     pub id: String,
//     pub username: String,
// }

/// Send request to create a new session
pub async fn create_session(
    client: &Client,
    //id: &str,
    username: &str,
) -> (StatusCode, Option<Value>) {
    let url = session::get_sessions_url();
    let params = Session {
        //id: id.to_string(),
        username: username.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::POST, url, None, Some(&params)).await;

    (status, json)
}

/// Send request to get all session
pub async fn view_all_sessions(
    client: &Client,
) -> (StatusCode, Option<Value>) {
    let url = session::get_sessions_url();

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, None, None::<()>).await;

    (status, json)
}

/// Send request to get all sessions by user
pub async fn view_sessions_by_user(
    client: &Client,
    username: &str,
) -> (StatusCode, Option<Value>) {
    let url = session::get_sessions_subpath_url("user", username);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, None, None::<()>).await;

    (status, json)
}

/// Send request to get a specific session by ID
pub async fn view_session_by_id(
    client: &Client,
    id: &str,
) -> (StatusCode, Option<Value>) {
    let url = session::get_sessions_subpath_url("id", id);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(id), None::<()>).await;

    (status, json)
}

/// Send request to partially or fully update a session
pub async fn update_session(
    client: &Client,
    id: &str,
    username: &str,
) -> (StatusCode, Option<Value>) {
    let url = session::get_sessions_exp_url(id);
    let params = Session {
        //id: id.to_string(),
        username: username.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::PATCH, url, Some(id), Some(&params)).await;

    (status, json)
}

/// Send request to delete a session by ID
pub async fn delete_session(
    client: &Client,
    id: &str,
) -> (StatusCode, Option<Value>) {
    let url = session::get_sessions_exp_url(id);

    let (status, json, _headers) =
        send_request(client, &Method::DELETE, url, Some(id), None::<()>).await;

    (status, json)
}

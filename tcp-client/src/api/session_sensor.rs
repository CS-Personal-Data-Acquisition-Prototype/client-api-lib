//! Requests for the session sensor endpoint

use crate::path::session;
use crate::requests::send_request::send_request;
use reqwest_wasm::{Client, Method, StatusCode};
use serde::Serialize;
use serde_json::Value;

/// Struct defining a session sensor
#[derive(Debug, Serialize)]
pub struct SessionSensor {
    pub session_id: String,
    pub sensor_id: String,
}

/// Send request to link a new sensor to a session
pub async fn create_session_sensor(
    client: &Client,
    session_id: &str,
    sensor_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = session::get_session_sensors_url();
    let params = SessionSensor {
        session_id: session_id.to_string(),
        sensor_id: sensor_id.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::POST, url, Some(session_id), Some(&params)).await;

    (status, json)
}

/// Send request to get all session sensor linkages
pub async fn view_all_sensor_sessions(
    client: &Client,
    session_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = session::get_session_sensors_url();

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await;

    (status, json)
}

/// Send request to get all sensors linked to a specific session
pub async fn view_sensors_by_session_id(
    client: &Client,
    session_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = session::get_session_sensors_subpath_url("session", session_id);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await;

    (status, json)
}

/// Send request to get a specific session sensor linkage by sensor ID
pub async fn view_session_sensor_by_sensor_id(
    client: &Client,
    session_id: &str,
    sensor_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = session::get_session_sensors_subpath_url("session-sensor", sensor_id);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await;

    (status, json)
}

/// Send request to partially or fully udpate a session sensor link
pub async fn update_sensor_session(
    client: &Client,
    session_id: &str,
    sensor_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = session::get_session_sensors_id_url(sensor_id);
    let params = SessionSensor {
        session_id: session_id.to_string(),
        sensor_id: sensor_id.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::PATCH, url, Some(session_id), Some(&params)).await;

    (status, json)
}

/// Send request to delete a session sensor linkage by ID
pub async fn delete_sensor_session(
    client: &Client,
    session_id: &str,
    sensor_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = session::get_session_sensors_id_url(sensor_id);

    let (status, json, _headers) =
        send_request(client, &Method::DELETE, url, Some(session_id), None::<()>).await;

    (status, json)
}

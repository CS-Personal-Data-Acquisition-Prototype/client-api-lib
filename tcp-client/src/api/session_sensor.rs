#![allow(dead_code)]

use crate::path::Path;
use crate::requests::send_request::send_request;
use reqwest::{Client, Method, StatusCode};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct SessionSensor {
    pub session_id: String,
    pub sensor_id: String,
}

pub async fn create_session_sensor(
    client: &Client,
    path: &Path,
    session_id: &str,
    sensor_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_session_sensors_url();
    let params = SessionSensor {
        session_id: session_id.to_string(),
        sensor_id: sensor_id.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::POST, url, Some(session_id), Some(&params)).await;

    (status, json)
}

pub async fn view_all_sensor_sessions(
    client: &Client,
    path: &Path,
    session_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_session_sensors_url();

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await;

    (status, json)
}

pub async fn view_sensors_by_session_id(
    client: &Client,
    path: &Path,
    session_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_session_sensors_subpath_url("session", session_id);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await;

    (status, json)
}

pub async fn view_session_sensor_by_sensor_id(
    client: &Client,
    path: &Path,
    session_id: &str,
    sensor_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_session_sensors_subpath_url("session-sensor", sensor_id);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await;

    (status, json)
}

pub async fn update_sensor_session(
    client: &Client,
    path: &Path,
    session_id: &str,
    sensor_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_session_sensors_id_url(sensor_id);
    let params = SessionSensor {
        session_id: session_id.to_string(),
        sensor_id: sensor_id.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::PATCH, url, Some(session_id), Some(&params)).await;

    (status, json)
}

pub async fn delete_sensor_session(
    client: &Client,
    path: &Path,
    session_id: &str,
    sensor_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_session_sensors_id_url(sensor_id);

    let (status, json, _headers) =
        send_request(client, &Method::DELETE, url, Some(session_id), None::<()>).await;

    (status, json)
}

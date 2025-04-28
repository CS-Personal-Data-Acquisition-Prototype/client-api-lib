//! Requests for the sensor endpoint

#![allow(dead_code)]

use crate::path::Path;
use crate::requests::send_request::send_request;
use reqwest::{Client, Method, StatusCode};
use serde::Serialize;
use serde_json::Value;

/// Struct defining a sensor
#[derive(Debug, Serialize)]
pub struct Sensor {
    #[serde(rename = "type")]
    pub sensor_type: String,
}

/// Send request to create a new sensor
pub async fn create_sensor(
    client: &Client,
    path: &Path,
    session_id: &str,
    sensor_type: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_sensor_url();
    let params = Sensor {
        sensor_type: sensor_type.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::POST, url, Some(session_id), Some(&params)).await;

    (status, json)
}

/// Send request to get all sensors
pub async fn view_all_sensors(
    client: &Client,
    path: &Path,
    session_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_sensor_url();

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await;

    (status, json)
}

/// Send request to get a specific sensor according to given ID
pub async fn view_sensor_by_id(
    client: &Client,
    path: &Path,
    session_id: &str,
    sensor_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_sensor_id_url(sensor_id);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await;

    (status, json)
}

/// Send request to partially or fully update a sensor
pub async fn update_sensor(
    client: &Client,
    path: &Path,
    session_id: &str,
    sensor_id: &str,
    sensor_type: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_sensor_id_url(sensor_id);
    let params = Sensor {
        sensor_type: sensor_type.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::PATCH, url, Some(session_id), Some(&params)).await;

    (status, json)
}

/// Send request to delete a sensor according to given ID
pub async fn delete_sensor(
    client: &Client,
    path: &Path,
    session_id: &str,
    sensor_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_sensor_id_url(sensor_id);

    let (status, json, _headers) =
        send_request(client, &Method::DELETE, url, Some(session_id), None::<()>).await;

    (status, json)
}

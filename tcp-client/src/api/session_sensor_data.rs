//! Requests for the session sensor data endpoint

#![allow(dead_code)]

use crate::path::Path;
use crate::requests::send_request::send_request;
use reqwest::{Client, Method, StatusCode};
use serde::Serialize;
use serde_json::Value;

/// Struct defining a batch of datapoints
#[derive(Debug, Serialize)]
pub struct Batch {
    pub datapoints: Vec<SessionSensorData>,
}

/// Struct defining a single datapoint
#[derive(Debug, Serialize)]
pub struct SessionSensorData {
    pub id: String,
    pub datetime: String,
    pub data_blob: String,
}

/// Send request to create a new datapoint
pub async fn create_datapoint(
    client: &Client,
    path: &Path,
    session_id: &str,
    id: &str,
    datetime: &str,
    data_blob: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_datapoint_url();
    let params = SessionSensorData {
        id: id.to_string(),
        datetime: datetime.to_string(),
        data_blob: data_blob.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::POST, url, Some(session_id), Some(&params)).await;

    (status, json)
}

/// Send request to batch create new datapoints
pub async fn batch_create_datapoint(
    client: &Client,
    path: &Path,
    session_id: &str,
    datapoints: Vec<SessionSensorData>,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_batch_url();
    let params = Batch { datapoints };

    let (status, json, _headers) =
        send_request(client, &Method::POST, url, Some(session_id), Some(&params)).await;

    (status, json)
}

/// Send request to get all datapoints
pub async fn view_all_datapoints(
    client: &Client,
    path: &Path,
    session_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_datapoint_url();

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await;

    (status, json)
}

/// Send request to get all datapoints linked to a given session
pub async fn view_datapoints_by_session_id(
    client: &Client,
    path: &Path,
    session_id: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_datapoint_subpath_url("session", session_id);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await;

    (status, json)
}

/// Send request to get all datapoints by session sensor ID
pub async fn view_datapoints_by_session_sensor(
    client: &Client,
    path: &Path,
    session_id: &str,
    id: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_datapoint_subpath_url("id", id);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await;

    (status, json)
}

/// Send request to get a specific datapoint
pub async fn view_datapoints_by_id_datetime(
    client: &Client,
    path: &Path,
    session_id: &str,
    id: &str,
    datetime: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_datapoint_subpath_url(id, datetime);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await;

    (status, json)
}

/// Send request to partially or fully udpate a specific datapoint
pub async fn update_datapoint(
    client: &Client,
    path: &Path,
    session_id: &str,
    id: &str,
    datetime: &str,
    data_blob: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_datapoint_subpath_url(id, datetime);
    let params = SessionSensorData {
        id: id.to_string(),
        datetime: datetime.to_string(),
        data_blob: data_blob.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::PATCH, url, Some(session_id), Some(&params)).await;

    (status, json)
}

/// Send request to delete a specific datapoint
pub async fn delete_datapoint(
    client: &Client,
    path: &Path,
    session_id: &str,
    id: &str,
    datetime: &str,
) -> (StatusCode, Option<Value>) {
    let url = &path.get_datapoint_subpath_url(id, datetime);

    let (status, json, _headers) =
        send_request(client, &Method::DELETE, url, Some(session_id), None::<()>).await;

    (status, json)
}

#![allow(dead_code)]

use crate::config::Config;
use crate::requests::send_request::send_request;
use reqwest::{Client, Method, StatusCode};
use serde::Serialize;
use serde_json::{to_string_pretty, Value};
use std::error::Error;

#[derive(Debug, Serialize)]
pub struct Batch {
    pub datapoints: Vec<SessionSensorData>,
}

#[derive(Debug, Serialize)]
pub struct SessionSensorData {
    pub id: String,
    pub datetime: String,
    pub data_blob: String,
}

pub async fn create_datapoint(
    client: &Client,
    config: &Config,
    session_id: &str,
    id: &str,
    datetime: &str,
    data_blob: &str,
) -> Result<(StatusCode, Option<Value>), Box<dyn Error>> {
    let url = &config.get_datapoint_url();
    let params = SessionSensorData {
        id: id.to_string(),
        datetime: datetime.to_string(),
        data_blob: data_blob.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::POST, url, Some(session_id), Some(&params)).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", to_string_pretty(&json_body).unwrap());
        Ok((status, Some(json_body)))
    } else {
        Ok((status, None))
    }
}

pub async fn batch_create_datapoint(
    client: &Client,
    config: &Config,
    session_id: &str,
    datapoints: Vec<SessionSensorData>,
) -> Result<(StatusCode, Option<Value>), Box<dyn Error>> {
    let url = &config.get_batch_url();
    let params = Batch { datapoints };

    let (status, json, _headers) =
        send_request(client, &Method::POST, url, Some(session_id), Some(&params)).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", to_string_pretty(&json_body).unwrap());
        Ok((status, Some(json_body)))
    } else {
        Ok((status, None))
    }
}

pub async fn view_all_datapoints(
    client: &Client,
    config: &Config,
    session_id: &str,
) -> Result<(StatusCode, Option<Value>), Box<dyn Error>> {
    let url = &config.get_datapoint_url();

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", to_string_pretty(&json_body).unwrap());
        Ok((status, Some(json_body)))
    } else {
        Ok((status, None))
    }
}

pub async fn view_datapoints_by_session_id(
    client: &Client,
    config: &Config,
    session_id: &str,
) -> Result<(StatusCode, Option<Value>), Box<dyn Error>> {
    let url = &config.get_datapoint_subpath_url("session", session_id);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", to_string_pretty(&json_body).unwrap());
        Ok((status, Some(json_body)))
    } else {
        Ok((status, None))
    }
}

pub async fn view_datapoints_by_session_sensor(
    client: &Client,
    config: &Config,
    session_id: &str,
    id: &str,
) -> Result<(StatusCode, Option<Value>), Box<dyn Error>> {
    let url = &config.get_datapoint_subpath_url("id", id);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", to_string_pretty(&json_body).unwrap());
        Ok((status, Some(json_body)))
    } else {
        Ok((status, None))
    }
}

pub async fn view_datapoints_by_id_datetime(
    client: &Client,
    config: &Config,
    session_id: &str,
    id: &str,
    datetime: &str,
) -> Result<(StatusCode, Option<Value>), Box<dyn Error>> {
    let url = &config.get_datapoint_subpath_url(id, datetime);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", to_string_pretty(&json_body).unwrap());
        Ok((status, Some(json_body)))
    } else {
        Ok((status, None))
    }
}

pub async fn update_datapoint(
    client: &Client,
    config: &Config,
    session_id: &str,
    id: &str,
    datetime: &str,
    data_blob: &str,
) -> Result<(StatusCode, Option<Value>), Box<dyn Error>> {
    let url = &config.get_datapoint_subpath_url(id, datetime);
    let params = SessionSensorData {
        id: id.to_string(),
        datetime: datetime.to_string(),
        data_blob: data_blob.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::PATCH, url, Some(session_id), Some(&params)).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", to_string_pretty(&json_body).unwrap());
        Ok((status, Some(json_body)))
    } else {
        Ok((status, None))
    }
}

pub async fn delete_datapoint(
    client: &Client,
    config: &Config,
    session_id: &str,
    id: &str,
    datetime: &str,
) -> Result<(StatusCode, Option<Value>), Box<dyn Error>> {
    let url = &config.get_datapoint_subpath_url(id, datetime);

    let (status, json, _headers) =
        send_request(client, &Method::DELETE, url, Some(session_id), None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", to_string_pretty(&json_body).unwrap());
        Ok((status, Some(json_body)))
    } else {
        Ok((status, None))
    }
}

#![allow(dead_code)]

use crate::config::Config;
use crate::requests::send_request::send_request;
use reqwest::{Client, Method, StatusCode};
use serde::Serialize;
use serde_json::{to_string_pretty, Value};
use std::error::Error;

#[derive(Debug, Serialize)]
pub struct SessionSensor {
    pub session_id: String,
    pub sensor_id: String,
}

pub async fn create_session_sensor(
    client: &Client,
    config: &Config,
    session_id: &str,
    sensor_id: &str,
) -> Result<(StatusCode, Option<Value>), Box<dyn Error>> {
    let url = &config.get_session_sensors_url();
    let params = SessionSensor {
        session_id: session_id.to_string(),
        sensor_id: sensor_id.to_string(),
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

pub async fn view_all_sensor_sessions(
    client: &Client,
    config: &Config,
    session_id: &str,
) -> Result<(StatusCode, Option<Value>), Box<dyn Error>> {
    let url = &config.get_session_sensors_url();

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

pub async fn view_sensors_by_session_id(
    client: &Client,
    config: &Config,
    session_id: &str,
) -> Result<(StatusCode, Option<Value>), Box<dyn Error>> {
    let url = &config.get_session_sensors_subpath_url("session", session_id);

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

pub async fn view_session_sensor_by_sensor_id(
    client: &Client,
    config: &Config,
    session_id: &str,
    sensor_id: &str,
) -> Result<(StatusCode, Option<Value>), Box<dyn Error>> {
    let url = &config.get_session_sensors_subpath_url("session-sensor", sensor_id);

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

pub async fn update_sensor_session(
    client: &Client,
    config: &Config,
    session_id: &str,
    sensor_id: &str,
) -> Result<(StatusCode, Option<Value>), Box<dyn Error>> {
    let url = &config.get_session_sensors_id_url(sensor_id);
    let params = SessionSensor {
        session_id: session_id.to_string(),
        sensor_id: sensor_id.to_string(),
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

pub async fn delete_sensor_session(
    client: &Client,
    config: &Config,
    session_id: &str,
    sensor_id: &str,
) -> Result<(StatusCode, Option<Value>), Box<dyn Error>> {
    let url = &config.get_session_sensors_id_url(sensor_id);

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

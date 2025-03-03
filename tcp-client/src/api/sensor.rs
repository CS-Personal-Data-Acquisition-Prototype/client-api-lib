#![allow(dead_code)]

use crate::config::Config;
use crate::requests::send_request::send_request;
use reqwest::{Client, Method};
use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize)]
pub struct Sensor {
    #[serde(rename = "type")]
    pub sensor_type: String,
}

// TODO: Change the return type, need to receive response

pub async fn create_sensor(
    client: &Client,
    config: &Config,
    session_id: &str,
    sensor_type: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_sensors_url();
    let params = Sensor {
        sensor_type: sensor_type.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::POST, url, Some(session_id), Some(&params)).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    Ok(())
}

pub async fn view_all_sensors(
    client: &Client,
    config: &Config,
    session_id: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_sensor_url();

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    Ok(())
}

pub async fn view_sensor_by_id(
    client: &Client,
    config: &Config,
    session_id: &str,
    sensor_id: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_sensor_id_url(sensor_id);

    let (status, json, _headers) =
        send_request(client, &Method::GET, url, Some(session_id), None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    Ok(())
}

pub async fn update_sensor(
    client: &Client,
    config: &Config,
    session_id: &str,
    sensor_id: &str,
    sensor_type: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_sensor_id_url(sensor_id);
    let params = Sensor {
        sensor_type: sensor_type.to_string(),
    };

    let (status, json, _headers) =
        send_request(client, &Method::PATCH, url, Some(session_id), Some(&params)).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    Ok(())
}

pub async fn delete_sensor(
    client: &Client,
    config: &Config,
    session_id: &str,
    sensor_id: &str,
) -> Result<(), Box<dyn Error>> {
    let url = &config.get_sensor_id_url(sensor_id);

    let (status, json, _headers) =
        send_request(client, &Method::DELETE, url, Some(session_id), None::<()>).await?;

    println!("Response status: {}", status);

    if let Some(json_body) = json {
        println!("{}", serde_json::to_string_pretty(&json_body).unwrap());
    }

    Ok(())
}

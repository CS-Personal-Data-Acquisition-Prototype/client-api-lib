use reqwest::{
    header::{HeaderName, HeaderValue, CONTENT_TYPE},
    Client, Method,
};
use serde::Serialize;
use serde_json;
use std::error::Error;

pub async fn send_request<T>(
    client: &Client,
    method: &Method,
    url: &str,
    session_id: Option<&str>,
    body: Option<T>,
) -> Result<(reqwest::StatusCode, serde_json::Value), Box<dyn Error>>
where
    T: Serialize,
{
    let mut request = client.request(method.clone(), url);

    // Add CONTENT_TYPE header for POST and PATCH methods
    if *method == Method::POST || *method == Method::PATCH {
        request = request.header(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    }

    // Add session_id header if provided
    if let Some(session_id) = session_id {
        request = request.header(
            HeaderName::from_static("session_id"),
            HeaderValue::from_str(session_id)?,
        );
    }

    // Add request body if provided
    if let Some(body) = body {
        request = request.json(&body);
    }

    // Send request and get status code and response body
    let res = request.send().await?;
    let status = res.status();
    let json: serde_json::Value = res.json().await?;

    Ok((status, json))
}

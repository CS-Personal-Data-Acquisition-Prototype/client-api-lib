use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE, COOKIE},
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
) -> Result<(reqwest::StatusCode, Option<serde_json::Value>, HeaderMap), Box<dyn Error>>
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
            COOKIE,
            HeaderValue::from_str(&format!("session_id={}", session_id))?,
        );
    }

    // Add request body if provided
    if let Some(body) = body {
        request = request.json(&body);
    }

    // Send request and get status code and response body
    let res = request.send().await?;
    let status = res.status();
    let headers = res.headers().clone();

    // Receive json body if not No Content
    if status != reqwest::StatusCode::NO_CONTENT {
        let json: serde_json::Value = res.json().await?;
        Ok((status, Some(json), headers))
    } else {
        Ok((status, None, headers))
    }
}

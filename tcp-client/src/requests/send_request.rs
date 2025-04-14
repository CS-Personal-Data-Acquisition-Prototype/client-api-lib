use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE, COOKIE},
    Client, Method,
};
use serde::Serialize;
use serde_json;

pub async fn send_request<T>(
    client: &Client,
    method: &Method,
    url: &str,
    session_id: Option<&str>,
    body: Option<T>,
) -> (reqwest::StatusCode, Option<serde_json::Value>, HeaderMap)
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
        match HeaderValue::from_str(&format!("session_id={}", session_id)) {
            Ok(value) => {
                request = request.header(COOKIE, value);
            }
            Err(e) => {
                eprintln!("Failed to parse session_id header: {}", e);
            }
        }
    }

    // Add request body if provided
    if let Some(body) = body {
        request = request.json(&body);
    }

    // Send request and handle all errors
    let res = match request.send().await {
        Ok(response) => response,
        Err(_) => {
            return (
                reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                None,
                HeaderMap::new(),
            );
        }
    };

    // Get status and headers from the response
    let status = res.status();
    let headers = res.headers().clone();

    // Receive json body if not No Content
    let json = if status != reqwest::StatusCode::NO_CONTENT {
        match res.json::<serde_json::Value>().await {
            Ok(body) => Some(body),
            Err(_) => None,
        }
    } else {
        None
    };

    (status, json, headers)
}

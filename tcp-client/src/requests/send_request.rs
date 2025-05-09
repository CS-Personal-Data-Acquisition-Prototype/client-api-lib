use reqwest_wasm::{
    header::{HeaderMap, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE, COOKIE},
    Client, Method,
};
use serde::Serialize;
use serde_json;

pub async fn send_request<T>(
    client: &Client,
    method: &Method,
    url: String,
    session_id: Option<&str>,
    body: Option<T>,
) -> (reqwest_wasm::StatusCode, Option<serde_json::Value>, HeaderMap)
where
    T: Serialize,
{
    let mut request = client.request(method.clone(), url);

    // Add content-type header for POST and PATCH methods
    if *method == Method::POST || *method == Method::PATCH {
        request = request.header(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    }

    // Add session_id in cookie header if provided
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

    // Check if there is a body to send in the request
    if let Some(body) = body {
        // Serialize the body and get the length
        let serialized_body = serde_json::to_vec(&body).unwrap();
        let content_length = serialized_body.len();
        
        // Add content-length header
        request = request.header(CONTENT_LENGTH, HeaderValue::from_str(&content_length.to_string()).unwrap());

        // Add the json body to the request
        request = request.json(&body);
    } else {
        request = request.header(CONTENT_LENGTH, 0);
    }

    // Send request
    let res = match request.send().await {
        Ok(response) => response,
        Err(_) => {
            return (
                reqwest_wasm::StatusCode::INTERNAL_SERVER_ERROR,
                None,
                HeaderMap::new(),
            );
        }
    };

    // Get status and headers from the response
    let status = res.status();
    let headers = res.headers().clone();

    // Receive json body if not No Content
    let json = if status != reqwest_wasm::StatusCode::NO_CONTENT {
        match res.json::<serde_json::Value>().await {
            Ok(body) => Some(body),
            Err(_) => None,
        }
    } else {
        None
    };

    (status, json, headers)
}
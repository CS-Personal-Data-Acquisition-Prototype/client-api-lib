mod api;
mod config;
mod requests;

use api::{auth, sensor, session, session_sensor, session_sensor_data, user};
use config::Config;
use reqwest::Client;
use tokio;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let config = Config::new();

    run_demo(client, config).await;
}

pub async fn run_demo(client: Client, config: Config) {
    // Demo for user operations
    println!("--- USER OPERATIONS ---");

    let username = "user_1";
    let pw = "pass_1";
    let session_id = "session_id_1";

    println!("Creating user...");
    let mut _res = user::create_user(&client, &config, username, pw).await;

    println!("\nLogging in user...");
    let mut _res2 = auth::user_login(&client, &config, "user_1", "pass_1").await;

    println!("\nRenewing session...");
    _res2 = auth::renew_session(&client, &config, session_id).await;

    println!("\nViewing user profile...");
    _res = user::view_user_profile(&client, &config, session_id).await;

    println!("\nDeleting user...");
    _res = user::delete_user(&client, &config, username).await;

    // Demo for sensor operations
    println!("\n--- SENSOR OPERATIONS ---");

    let sensor_type = "sensor_type";
    let sensor_id = "id_1";

    println!("Creating sensor...");
    _res = sensor::create_sensor(&client, &config, session_id, sensor_type).await;

    println!("\nViewing all sensors...");
    _res = sensor::view_all_sensors(&client, &config, session_id).await;

    println!("\nViewing sensor by ID...");
    _res = sensor::view_sensor_by_id(&client, &config, session_id, sensor_id).await;

    println!("\nDeleting sensor...");
    _res = sensor::delete_sensor(&client, &config, session_id, sensor_id).await;

    // Demo for session operations
    println!("\n--- SESSION OPERATIONS ---");

    let username = "user_1";
    let session_id = "session_id_1";

    println!("Creating session...");
    _res = session::create_session(&client, &config, session_id, username).await;

    println!("\nViewing all sessions...");
    _res = session::view_all_sessions(&client, &config, session_id).await;

    println!("\nViewing sessions by user...");
    _res = session::view_sessions_by_user(&client, &config, session_id, username).await;

    println!("\nDeleting session...");
    _res = session::delete_session(&client, &config, session_id).await;

    // Demo for session sensor operations
    println!("\n--- SESSION SENSOR OPERATIONS ---");

    let sensor_id = "sensor_id_1";

    println!("Creating session sensor...");
    _res = session_sensor::create_session_sensor(&client, &config, session_id, sensor_id).await;

    println!("\nViewing all session sensors...");
    _res = session_sensor::view_all_sensor_sessions(&client, &config, session_id).await;

    println!("\nViewing session sensor by sensor ID...");
    _res =
        session_sensor::view_session_sensor_by_sensor_id(&client, &config, session_id, sensor_id)
            .await;

    println!("\nUpdating session sensor...");
    _res = session_sensor::update_sensor_session(&client, &config, session_id, sensor_id).await;

    println!("\nDeleting session sensor...");
    _res = session_sensor::delete_sensor_session(&client, &config, session_id, sensor_id).await;

    // Demo for datapoint operations
    println!("\n--- DATAPOINT OPERATIONS ---");

    let id = "id_1";
    let datetime = "datetime_1";
    let data_blob = "data_blob_1";

    let datapoints = vec![
        session_sensor_data::SessionSensorData {
            id: "id_11".to_string(),
            datetime: "datetime_11".to_string(),
            data_blob: "data_blob_11".to_string(),
        },
        session_sensor_data::SessionSensorData {
            id: "id_22".to_string(),
            datetime: "datetime_22".to_string(),
            data_blob: "data_blob_22".to_string(),
        },
    ];

    println!("Creating datapoint...");
    _res = session_sensor_data::create_datapoint(
        &client, &config, session_id, id, datetime, data_blob,
    )
    .await;

    println!("\nBatch creating datapoints...");
    _res =
        session_sensor_data::batch_create_datapoint(&client, &config, session_id, datapoints).await;

    println!("\nViewing all datapoints...");
    _res = session_sensor_data::view_all_datapoints(&client, &config, session_id).await;

    println!("\nUpdating datapoint...");
    _res = session_sensor_data::update_datapoint(
        &client, &config, session_id, id, datetime, data_blob,
    )
    .await;

    println!("\nDeleting datapoint...");
    _res = session_sensor_data::delete_datapoint(&client, &config, session_id, id, datetime).await;
}

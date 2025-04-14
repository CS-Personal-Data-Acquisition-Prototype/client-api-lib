#![allow(dead_code)]

use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Path {
    pub base_url: String,
}

impl Path {
    pub fn new() -> Self {
        dotenv().ok();
        let base_url =
            env::var("API_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:7878".to_string());

        Path { base_url }
    }

    pub fn get_user_url(&self) -> String {
        format!("{}/users", self.base_url)
    }

    pub fn get_profile_url(&self) -> String {
        format!("{}/users/profile", self.base_url)
    }

    pub fn get_username_url(&self, username: &str) -> String {
        format!("{}/users/{}", self.base_url, username)
    }

    pub fn get_login_url(&self) -> String {
        format!("{}/authentication/login", self.base_url)
    }

    pub fn get_logout_url(&self) -> String {
        format!("{}/authentication/logout", self.base_url)
    }

    pub fn get_renew_url(&self) -> String {
        format!("{}/authentication/renew", self.base_url)
    }

    pub fn get_sensor_url(&self) -> String {
        format!("{}/sensors", self.base_url)
    }

    pub fn get_sensor_id_url(&self, sensor_id: &str) -> String {
        format!("{}/sensors/{}", self.base_url, sensor_id)
    }

    pub fn get_sessions_url(&self) -> String {
        format!("{}/sessions", self.base_url)
    }

    pub fn get_sessions_exp_url(&self, endpoint: &str) -> String {
        format!("{}/sessions/{}", self.base_url, endpoint)
    }

    pub fn get_sessions_subpath_url(&self, subpath: &str, endpoint: &str) -> String {
        format!("{}/sessions/{}/{}", self.base_url, subpath, endpoint)
    }

    pub fn get_session_sensors_url(&self) -> String {
        format!("{}/sessions-sensors", self.base_url)
    }

    pub fn get_session_sensors_id_url(&self, id: &str) -> String {
        format!("{}/sessions-sensors/{}", self.base_url, id)
    }

    pub fn get_session_sensors_subpath_url(&self, subpath: &str, id: &str) -> String {
        format!("{}/sessions-sensors/{}/{}", self.base_url, subpath, id)
    }

    pub fn get_datapoint_url(&self) -> String {
        format!("{}/sessions-sensors-data", self.base_url)
    }

    pub fn get_batch_url(&self) -> String {
        format!("{}/sessions-sensors-data/batch", self.base_url)
    }

    pub fn get_datapoint_subpath_url(&self, subpath: &str, id: &str) -> String {
        format!("{}/sessions-sensors-data/{}/{}", self.base_url, subpath, id)
    }
}

use std::env;

/// Get the API base URL from an environment variable
fn get_base_url() -> String {
    dotenv::dotenv().ok();
    env::var("API_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:7878".to_string())
}

/// Module for obtaining User related endpoint URLs
pub mod user {
    use super::get_base_url;

    pub fn get_user_url() -> String {
        let base_url = get_base_url();
        format!("{}/users", base_url)
    }

    pub fn get_profile_url() -> String {
        let base_url = get_base_url();
        format!("{}/users/profile", base_url)
    }

    pub fn get_username_url(username: &str) -> String {
        let base_url = get_base_url();
        format!("{}/users/{}", base_url, username)
    }
}

/// Module for obtaining authentication related endpoint URLs
pub mod auth {
    use super::get_base_url;

    pub fn get_login_url() -> String {
        let base_url = get_base_url();
        format!("{}/authentication/login", base_url)
    }

    pub fn get_logout_url() -> String {
        let base_url = get_base_url();
        format!("{}/authentication/logout", base_url)
    }

    pub fn get_renew_url() -> String {
        let base_url = get_base_url();
        format!("{}/authentication/renew", base_url)
    }
}

/// Module for obtaining sensor related endpoint URLs
pub mod sensor {
    use super::get_base_url;

    pub fn get_sensor_url() -> String {
        let base_url = get_base_url();
        format!("{}/sensors", base_url)
    }

    pub fn get_sensor_id_url(sensor_id: &str) -> String {
        let base_url = get_base_url();
        format!("{}/sensors/{}", base_url, sensor_id)
    }
}

/// Module for obtaining session related endpoint URLs
pub mod session {
    use super::get_base_url;

    pub fn get_sessions_url() -> String {
        let base_url = get_base_url();
        format!("{}/sessions", base_url)
    }

    pub fn get_sessions_exp_url(endpoint: &str) -> String {
        let base_url = get_base_url();
        format!("{}/sessions/{}", base_url, endpoint)
    }

    pub fn get_sessions_subpath_url(subpath: &str, endpoint: &str) -> String {
        let base_url = get_base_url();
        format!("{}/sessions/{}/{}", base_url, subpath, endpoint)
    }

    pub fn get_session_sensors_url() -> String {
        let base_url = get_base_url();
        format!("{}/sessions-sensors", base_url)
    }

    pub fn get_session_sensors_id_url(id: &str) -> String {
        let base_url = get_base_url();
        format!("{}/sessions-sensors/{}", base_url, id)
    }

    pub fn get_session_sensors_subpath_url(subpath: &str, id: &str) -> String {
        let base_url = get_base_url();
        format!("{}/sessions-sensors/{}/{}", base_url, subpath, id)
    }
}

/// Module for obtaining datapoint related endpoint URLs
pub mod datapoint {
    use super::get_base_url;

    pub fn get_datapoint_url() -> String {
        let base_url = get_base_url();
        format!("{}/sessions-sensors-data", base_url)
    }

    pub fn get_batch_url() -> String {
        let base_url = get_base_url();
        format!("{}/sessions-sensors-data/batch", base_url)
    }

    pub fn get_datapoint_subpath_url(subpath: &str, id: &str) -> String {
        let base_url = get_base_url();
        format!("{}/sessions-sensors-data/{}/{}", base_url, subpath, id)
    }

    pub fn get_datapoint_datetime_url(id: &str, datetime: &str) -> String {
        let base_url = get_base_url();
        format!("{}/sessions-sensors-data/session/{}/{}", base_url, id, datetime)
    }
}

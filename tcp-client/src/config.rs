use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub base_url: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        let base_url =
            env::var("API_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:7878".to_string());

        Config { base_url }
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
}

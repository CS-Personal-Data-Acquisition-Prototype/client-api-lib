use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    Client, Url,
};
use serde::Serialize;
use serde_json;
use tokio;

// TODO: Find out how to not make a new Client every time
// TODO: Cookie jar implementation

#[derive(Debug, Serialize)]
struct UserPassword {
    username: String,
    password_hash: String,
}

#[derive(Debug, Serialize)]
struct SessionId {
    session_id: String,
}

// Passwords cannot be same as username or less than ten characters long
fn valid_password(username: &str, password: &str) -> bool {
    if username == password || password.chars().count() < 10 {
        return false;
    } else {
        return true;
    }
}

fn hash_pw(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let argon2id = Argon2::default();

    match argon2id.hash_password(password, &salt) {
        Ok(hashed_pw) => hashed_pw.to_string(),
        Err(e) => panic!("Unable to hash password: {:?}", e),
    }
}

async fn create_user() -> Result<(), reqwest::Error> {
    let username = "username";
    let password = "workingpassword";

    if !valid_password(username, password) {
        // TODO: Should display password issues in GUI
        panic!("Invalid password");
    }

    let hashed = hash_pw(password.as_bytes());
    let params = UserPassword {
        username: username.to_string(),
        password_hash: hashed,
    };

    let client = Client::new();
    let res: serde_json::Value = client
        .post("http://127.0.0.1:7878/users")
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .json(&params)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", res);

    Ok(())
}

async fn view_all_users() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let res: serde_json::Value = client
        .get("http://127.0.0.1:7878/users")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", res);

    Ok(())
}

async fn view_user_profile() -> Result<(), reqwest::Error> {
    // TODO: Should be getting this from stored cookies?
    let session_id = String::from("skdfj");
    let params = SessionId {
        session_id: session_id,
    };

    let client = Client::new();
    let res: serde_json::Value = client
        .get("http://127.0.0.1:7878/users/profile")
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .json(&params)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", res);

    Ok(())
}

// Fix error part of Result
async fn view_user_by_username() -> Result<(), Box<dyn std::error::Error>> {
    let username = "username";
    let api_base = Url::parse(format!("http://127.0.0.1:7878/users/{username}").as_str())?;

    let client = Client::new();
    let res: serde_json::Value = client.get(api_base).send().await?.json().await?;

    println!("{:#?}", res);

    Ok(())
}

#[tokio::main]
async fn main() {
    // let client = Client::new();

    // let response = client
    //     .get("http://127.0.0.1:7878")
    //     .send()
    //     .await
    //     .expect("Failed to send request");

    // let _body = response.text().await.expect("Failed to read response body");
    // println!("Response: {}", body);

    let _res = create_user().await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;

    #[tokio::test]
    async fn test_create_user() {
        // let client = Client::new();

        // Create async server on correct port
        let opts = mockito::ServerOpts {
            host: "127.0.0.1",
            port: 7878,
            ..Default::default()
        };
        let mut server = mockito::Server::new_with_opts_async(opts).await;

        let m = server
            .mock("POST", "/users")
            .with_body("hiii")
            .create_async()
            .await;

        let res = create_user().await;

        // Note: Will print reqwest error because body is not json
        m.assert_async().await;
        println!("{m:#?}");
        println!("{:#?}", res);
    }
}

extern crate reqwest;
extern crate serde_json;

use serde_json::json;

pub async fn create_golfer_handler(
    username: &str,
    email_address: &str,
    password: &str,
) -> Result<(reqwest::StatusCode, String), reqwest::Error> {
    let http_client = reqwest::Client::new();
    let golfer_json = json!({
        "username": username,
        "email": email_address,
        "password": password
    });

    let response = http_client
        .post("http://localhost:8080/api/golfers")
        .header("Content-Type", "application/json")
        .body(golfer_json.to_string())
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;
    Ok((status, body))
}

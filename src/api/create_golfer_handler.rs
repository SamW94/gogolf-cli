extern crate reqwest; 
extern crate serde_json;

use serde_json::json;
use reqwest::Error;
use reqwest::Response;

pub async fn create_golfer_handler(username: &str, email_address: &str, password: &str) -> Result<Response, Error> {
    let http_client = reqwest::Client::new();
    let golfer_json = json!({
        "username": username,
        "email": email_address,
        "password": password
    });

    println!("{}", golfer_json.to_string());
    
    let response = http_client
        .post("http://localhost:7117/api/golfers")
        .header("Content-Type", "application/json")
        .body(golfer_json.to_string())
        .send()
        .await?;

    Ok(response)
}
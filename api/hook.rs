extern crate serde;
extern crate serde_json;

use http::StatusCode;
use serde_json::{json, Value};
use std::{env, error::Error};
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request, Response};

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    let payload: Value = serde_json::from_slice(&request.into_body()).unwrap();
    let event = &payload["event"];

    if event["type"] == "url_verification" {
        let response = Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/plain")
            .body(event["challenge"].to_string())
            .unwrap();

        return Ok(response);
    }

    if event["type"] == "emoji_changed" && event["subtype"] == "add" {
        let emoji_name = event["name"].as_str().unwrap();
        let body = json!({
           "text": format!("New Emoji added. `:{}:` :{}:", emoji_name, emoji_name),
        });
        let client = reqwest::blocking::Client::new();
        let _ = client
            .post(env::var("SLACK_WEBHOOK_URL").unwrap())
            .json(&body)
            .send();
    }

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(String::from(""))
        .unwrap();

    return Ok(response);
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}

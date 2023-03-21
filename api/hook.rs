use serde_json::{json, Value};
use std::env;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let payload: Value = serde_json::from_slice(&req.into_body()).unwrap();

    if payload["type"] == "url_verification" {
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/plain")
            .body(Body::Text(payload["challenge"].to_string()))?);
    }

    let event = &payload["event"];

    if event["type"] == "emoji_changed" && event["subtype"] == "add" {
        let emoji_name = event["name"].as_str().unwrap();
        let body = json!({
           "text": format!("New Emoji added. `:{}:` :{}:", emoji_name, emoji_name),
        });
        let client = reqwest::Client::new();

        client
            .post(env::var("SLACK_WEBHOOK_URL").unwrap())
            .json(&body)
            .send()
            .await?;
    }

    return Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(Body::from(""))?);
}

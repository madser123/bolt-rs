use std::net::SocketAddr;

use bolt_rs::{
    App,
    app::{
        Auth,
        Shortcut,
        AppResult,
    },
};
use serde_json::json;

async fn shortcut1(i: Shortcut) -> AppResult<()> {
    println!("{i:?}");
    Ok(())
}

async fn initialize_app(port: u16) -> tokio::task::JoinHandle<()> {
    dotenv::from_filename(".dev.env").ok();

    let auth = Auth::new(
        dotenv::var("SIGNING_SECRET").unwrap(), 
        dotenv::var("BOT_TOKEN").ok(), 
        None,
    );

    let app = App::new(auth)
        .address(SocketAddr::from(([127, 0, 0, 1], port)))
        .shortcut("shortcut1", shortcut1);

    tokio::spawn(async move { app.start().await })
}

async fn send_fake_payload(payload: String, port: u16) -> Result<reqwest::Response, reqwest::Error> {
    // Create new timestamp
    let ts = chrono::Local::now().timestamp().to_string();

    // Encrypt payload
    let hmac = hmac_sha256::HMAC::mac(
        format!("v0:{}:{}", ts, payload),
        dotenv::var("SIGNING_SECRET").unwrap(),
    );

    // Spawn task
    tokio::spawn(
        reqwest::Client::new()
            .post(format!("http://127.0.0.1:{port}"))
            .header("X-Slack-Request-Timestamp", &ts)
            .header("X-Slack-Signature", format!("v0={}", hex::encode(hmac)))
            .body(payload)
            .send()
    ).await.unwrap()
}

#[tokio::test]
async fn known_payload() {
    let port = 3000;

    // Initialize app
    let app = initialize_app(port).await;
    
    // Create payload
    let shortcut = format!("payload={}",  urlencoding::encode(&json!({
        "type":"shortcut",
        "team": {
            "id":     "some_id",
            "domain": "some_domain"
        },
        "user": {
            "id":       "some_id",
            "username": "some_user",
            "team_id":  "some_id"
        },
        "action_ts":   "some_action_ts",
        "trigger_id":  "some_trigger_id",
        "callback_id": "shortcut1",
    }).to_string()));

    // Send payload to app
    let response = send_fake_payload(shortcut, port).await.unwrap();

    // Close app thread
    app.abort();

    assert!(response.status() == 200)
}


#[tokio::test]
async fn unknown_payload() {
    let port = 3001;

    // Initialize app
    let app = initialize_app(port).await;
    
    // Create payload
    let shortcut = format!("payload={}",  urlencoding::encode(&json!({
        "type":"shortcut",
        "team": {
            "id":     "some_id",
            "domain": "some_domain"
        },
        "user": {
            "id":       "some_id",
            "username": "some_user",
            "team_id":  "some_id"
        },
        "action_ts":   "some_action_ts",
        "trigger_id":  "some_trigger_id",
        "callback_id": "some_callback",
    }).to_string()));

    // Send payload to app
    let response = send_fake_payload(shortcut, port).await.unwrap();
}

#[tokio::test]
#[should_panic]
async fn no_signing_secret() {
    let port = 3002;

    let auth = Auth::default();

    App::new(auth)
        .address(SocketAddr::from(([127, 0, 0, 1], port)))
        .start()
        .await;
}
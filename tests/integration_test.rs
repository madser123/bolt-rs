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

#[tokio::test]
async fn create_app() {
    dotenv::from_filename(".dev.env").ok();

    let auth = Auth::new(
        dotenv::var("SIGNING_SECRET").unwrap(), 
        dotenv::var("BOT_TOKEN").ok(), 
        dotenv::var("USER_TOKEN").ok()
    );

    let app = App::new(auth)
        .address(SocketAddr::from(([127, 0, 0, 1], 3000)))
        .shortcut("shortcut1", shortcut1);

    let thread = tokio::spawn(async move { app.start().await });

    let ts = chrono::Local::now().timestamp().to_string();

    let payload = format!("payload={}",  urlencoding::encode(&json!({
        "type":"shortcut",
        "team":{
            "id":"T04U7JG59TR",
            "domain":"bolt-rs"
        },
        "user":{
            "id":"U04V0P4QQPK",
            "username":"madsaj10",
            "team_id":"T04U7JG59TR"
        },
        "action_ts": "some_action_ts",
        "trigger_id": "some_trigger_id",
        "callback_id":"shortcut1",
    }).to_string()));

    let hmac = hmac_sha256::HMAC::mac(
        format!("v0:{}:{}", ts, payload),
        dotenv::var("SIGNING_SECRET").unwrap(),
    );

    let app_response = tokio::spawn(reqwest::Client::new()
        .post("http://127.0.0.1:3000")
        .header("X-Slack-Request-Timestamp", &ts)
        .header("X-Slack-Signature", format!("v0={}", hex::encode(hmac)))
        .body(payload)
        .send()).await.unwrap();

    assert!(app_response.unwrap().status() == 200)
}

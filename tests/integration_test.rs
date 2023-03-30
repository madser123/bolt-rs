
#[tokio::test]
#[allow(unused_must_use)]
async fn test_conversation() {
    // For developing locally
    dotenv::from_filename(".dev.env");

    // Get token
    let bot_token = dotenv::var("BOT_TOKEN").unwrap();

    bolt_rs::message::Message::new()
        .text("TEST")
        .channel("C050KF6QU2K")
        .post(&bot_token)
        .await.unwrap();
}
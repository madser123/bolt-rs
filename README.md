# Bolt-rs

A WIP Bolt-like SDK for writing Slack-integrations and/or Slack-Apps in Rust.

> **Note**
>
>This is an open-source, community-made project, which is not affiliated or associated with Slack or the official Bolt-SDK from Slack.

## Client
Using bolt-rs, writing Slack-apps is a breeze:
```rust
// Create a shortcut handler
async fn my_shortcut(i: Shortcut) -> AppResult<()> {
    // print the shortcut-data
    println!("{i:?}");
    Ok(())
}

#[tokio::main]
async fn main() {
    // Create authentication context
    let auth = Auth::new(
        // A signing secret is needed
        env::var("SIGNING_SECRET").unwrap(), 

        // Bot and user-tokens are optional
        env::var("BOT_TOKEN").ok(), 
        None,
    );

    // Configure app
    let app = App::new(auth)
        // Serve on address 127.0.0.1:8080
        .address(SocketAddr::from(([127, 0, 0, 1], 8080)))
        // Append the shortcut function as a handler for 
        // shortcuts with trigger-id "my_trigger"
        .shortcut("my_trigger", shortcut1);

    // Start listening for requests
    app.start().await;
}
```

The client is a wrapper of [Axum](https://github.com/tokio-rs/axum)s thin abstraction over [Hyper](https://github.com/hyperium/hyper).

### Interactions
The bolt-rs client will react to interactions from slack by using a predefined function, or "handler", for each "identifier" of the interactions.

An example of a handler for each interaction could be like so:
```rust 
async fn my_block_action_handler(i: BlockAction) -> AppResult<()> {
    // Handle block_actions
    Ok(())
}

async fn my_message_action_handler(i: MessageAction) -> AppResult<()> {
    // Handle message_actions
    Ok(())
}

async fn my_shortcut_handler(i: Shortcut) -> AppResult<()> {
    // Handle shortcut
    Ok(())
}

async fn my_view_close_handler(i: ViewClosed) -> AppResult<()> {
    // Handle view_closed
    Ok(())
}

async fn my_view_submission_handler(i: ViewSubmission) -> AppResult<()> {
    // Handle view_submission
    Ok(())
}
```



## Composition
Bolt-rs provides the "blocks" ecosystem for composing slack-messages:
```rust
use bolt_rs::{
    block, 
    composition,
    element,
}
```


This includes traits for creating `View`, `Block`, `Message` and `Element` "templates" for your types.
```rust
pub MyData {
    channel: String,
    string: String,
    number: i32,
}

impl AsBlocks for MyData {
    fn as_blocks(&self) -> bolt_rs::BoltResult<Blocks> {
        let mut blocks = Blocks::new();

        blocks.push(block::Header::new(Text::plain("My data!")))?;
        blocks.push(block::Divider::new())?;
        blocks.push(block::Section::new()
            .id("my_section")
            .field(Text::mrkdwn(&format!("*String:* {}", self.number)))
            .field(Text::mrkdwn(&format!("*Number:* {}", self.number)))
        )?;

        Ok(blocks)
    }
}

impl AsMessage for MyData {
    fn as_message(&self) -> bolt_rs::BoltResult<Message> {
        Ok(Message::new()
            .channel(&self.channel)
            .blocks(self.as_blocks()?)
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let data = MyData {
        channel: "CHANNELID".to_string(),
        string: "Hello, World!",
        number: 1337,
    };

    let msg = data.as_message().post().await?;

    Ok(())
}
```
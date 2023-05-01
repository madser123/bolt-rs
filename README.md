# Bolt-rs

A WIP bolt-like SDK for Slack-Apps written in Rust.

## Usage
Bolt-rs provides the "blocks" ecosystem for composing slack-messages:
```rust
use bolt_rs::{
    block, 
    composition,
    element,
}
```


And also traits for creating `Block`, `Message` and `Element` "templates" for your types.
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
        blocks.push(block::Divider::new(Text::plain("My data!")))?;
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
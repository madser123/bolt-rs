use crate::pre::*;
use block::Blocks;
use element::Elements;

/// Convert any type into a message
pub trait AsMessage {
    fn as_message(&self) -> BoltResult<Message>;
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Message {
    channel: String,
    blocks: Option<Blocks>,
    attachments: Option<Elements>,
    text: Option<String>,
}

impl Message {
    /// Creates a new message
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the text of the message
    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    /// Sets the channel for the message to be posted to
    pub fn channel(mut self, channel: &str) -> Self {
        self.channel = channel.to_string();
        self
    }

    /// Adds blocks to the message.
    pub fn blocks(mut self, blocks: Blocks) -> Self {
        self.blocks = Some(blocks);
        self
    }

    /// Adds attachments to the message
    pub fn attachments(mut self, attachments: Elements) -> Self {
        self.attachments = Some(attachments);
        self
    }

    /// Posts the message to slack.
    pub async fn post(self, token: &str) -> BoltResult<Self> {
        Request::post("chat.postMessage", token)
            .json(&self)
            .send()
            .await?
            .unpack()
    }
}


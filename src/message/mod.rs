use crate::pre::*;
use block::Blocks;
use element::Elements;
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Message {
    channel: String,
    blocks: Option<Blocks>,
    attachments: Option<Elements>,
    text: Option<String>,
}

impl Message {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn channel(mut self, channel: &str) -> Self {
        self.channel = channel.to_string();
        self
    }

    /// Adds blocks to the message.
    pub fn blocks(mut self, blocks: Blocks) -> Self {
        self.blocks = Some(blocks);
        self
    }

    pub fn attachments(mut self, attachments: Elements) -> Self {
        self.attachments = Some(attachments);
        self
    }

    pub async fn post(self, token: &str) -> BoltResult<Self> {
        Request::post("chat.postMessage", token)
            .json(&self)
            .send()
            .await?
            .unpack()
    }
}

pub trait AsMessage {
    fn as_message(&self) -> BoltResult<Message>;
}

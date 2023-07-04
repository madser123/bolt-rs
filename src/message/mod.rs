use crate::pre::{
    block::Blocks, element::Elements, skip_serializing_none, BoltResult, Deserialize, Request,
    Serialize,
};

/// Convert any type into a message
#[allow(clippy::module_name_repetitions)]
pub trait AsMessage {
    /// Turns `self` into a message, ready to be sent to slack.
    ///
    /// # Errors
    ///
    /// An error should occur if any blocks and/or text in the message fail to serialize.
    ///
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
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the text of the message
    #[must_use]
    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    /// Sets the channel for the message to be posted to
    #[must_use]
    pub fn channel(mut self, channel: &str) -> Self {
        self.channel = channel.to_string();
        self
    }

    /// Adds blocks to the message.
    #[must_use]
    pub fn blocks(mut self, blocks: Blocks) -> Self {
        self.blocks = Some(blocks);
        self
    }

    /// Adds attachments to the message
    #[must_use]
    pub fn attachments(mut self, attachments: Elements) -> Self {
        self.attachments = Some(attachments);
        self
    }

    /// Posts the message to slack.
    ///
    /// # Errors
    ///
    /// An error occurs if the request fails to be sent, or if slack reports any errors back.
    ///
    pub async fn post(self, token: &str) -> BoltResult<Self> {
        Request::post("chat.postMessage", token)
            .json(&self)
            .send()
            .await?
            .unpack()
    }
}

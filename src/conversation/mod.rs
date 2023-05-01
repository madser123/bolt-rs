use crate::pre::*;
use reqwest::multipart::Form;
use message::{Message, AsMessage};
use block::Blocks;
use element::Elements;

/// A slack-conversation
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct Conversation {
    pub id: String,
    pub created: Option<i64>,
    pub is_im: Option<bool>,
    pub is_org_shared: Option<bool>,
    pub user: Option<String>,
    pub last_read: Option<String>,
    //pub latest: Option<????>,
    pub unread_count: Option<i64>,
    pub unread_count_display: Option<i64>,
    pub is_open: Option<bool>,
    pub priority: Option<i32>,
}

impl Conversation {
    /// Returns a default [Starter] which can be used to open a conversation.
    pub fn open_new() -> Starter {
        Starter::default()
    }

    /// Returns a new [Starter] with a user assigned to it.
    pub fn user(user: &str) -> Starter {
        Starter::default().add_user(user)
    }

    /// Returns a new [Starter] with a channel assigned to it.
    pub fn channel(channel: &str) -> Starter {
        Starter::default().channel(channel)
    }

    /// Updates the current conversation.
    pub async fn update(self, token: &str) -> BoltResult<Self> {
        Request::post("conversations.join", token)
            .multipart(Form::new()
                .text("channel", self.id)    
            )
            .send()
            .await?
            .unpack()
    }

    /// Sends normal text to the opened conversation.
    pub async fn send_text(self, text: &str, token: &str) -> BoltResult<Self> {
        self.as_message()?
            .text(text)
            .post(token)
            .await?;

        self.update(token).await
    }

    /// Sends blocks to the opened conversation.
    pub async fn send_blocks(self, blocks: Blocks, token: &str) -> BoltResult<Self> {
        self.as_message()?
            .blocks(blocks)
            .post(token)
            .await?;

        self.update(token).await
    }

    /// Sends attachments to the opened conversation.
    pub async fn send_attachments(self, attachments: Elements, token: &str) -> BoltResult<Self> {
        self.as_message()?
            .attachments(attachments)
            .post(token)
            .await?;

        self.update(token).await
    }
}

impl AsMessage for Conversation {
    fn as_message(&self) -> BoltResult<Message> {
        Ok(Message::new().channel(&self.id))
    }
}

/// A simple copy of the `Conversation` object for sending
/// payloads to slack that starts or joins the conversations.
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Starter {
    channel: Option<String>,
    prevent_creation: Option<bool>,
    return_im: Option<bool>,
    users: Vec<String>,
}

impl Starter {
    /// Sends this starter to slack and returns the joined conversation.
    pub async fn start(self, token: &str) -> BoltResult<Conversation> {
        Request::post("conversations.open", token)
            .json(&self)
            .send()
            .await?
            .unpack()
    }

    /// Adds a user to the conversation.
    fn add_user(mut self, user: &str) -> Self {
        self.users.push(user.to_string());
        self
    }

    /// Adds a channel to the conversation.
    fn channel(mut self, channel: &str) -> Self {
        self.channel = Some(channel.to_string());
        self
    }

    /// TODO
    pub fn return_im(mut self, im: bool) -> Self {
        self.return_im = Some(im);
        self
    }

    /// TODO
    pub fn prevent_creation(mut self, prevent: bool) -> Self {
        self.prevent_creation = Some(prevent);
        self
    }
}

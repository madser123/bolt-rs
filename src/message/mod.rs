use crate::{
    pre::*,
    parsing::parse_response, 
    block::Blocks, 
    element::Elements
};

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct MessageResponse {
    ok: bool,
    channel: Option<String>,
    ts: Option<String>,
    error: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    pub channel: Option<String>,
    pub ts: Option<String>
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

    pub async fn post(self, token: &str) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let resp = client
            .post("https://slack.com/api/chat.postMessage")
            .bearer_auth(token)
            .json(&self)
            .send()
            .await?;

        let resp = parse_response::<MessageResponse>(resp).await?;

        if let Some(error) = resp.error {
            return Err(Error::Message(error))
        }

        Ok(Response {
            channel: resp.channel,
            ts: resp.ts,
        })
    }
}

pub trait MessageAble {
    fn into_message(self) -> Result<Message, Error>;
}

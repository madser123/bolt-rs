use crate::pre::*;
use message::Message;

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
    pub fn user(user: &str) -> Starter {
        Starter::default().add_user(user)
    }

    pub fn channel(channel: &str) -> Starter {
        Starter::default().channel(channel)
    }

    pub async fn update(self, token: &str) -> BoltResult<Self> {
        Request::post("conversations.join", token)
            .multipart(reqwest::multipart::Form::new()
                .text("channel", self.id)    
            )
            .send()
            .await?
            .unpack()
    }

    pub async fn send_text(self, text: &str, token: &str) -> BoltResult<Self> {
        Message::new()
            .text(text)
            .post(token)
            .await?;

        self.update(token).await
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Starter {
    channel: Option<String>,
    prevent_creation: Option<bool>,
    return_im: Option<bool>,
    users: Vec<String>,
}

impl Starter {
    pub async fn start(self, token: &str) -> BoltResult<Conversation> {
        Request::post("conversations.open", token)
            .json(&self)
            .send()
            .await?
            .unpack()
    }

    fn add_user(mut self, user: &str) -> Self {
        self.users.push(user.to_string());
        self
    }

    fn channel(mut self, channel: &str) -> Self {
        self.channel = Some(channel.to_string());
        self
    }

    pub fn return_im(mut self, im: bool) -> Self {
        self.return_im = Some(im);
        self
    }

    pub fn prevent_creation(mut self, prevent: bool) -> Self {
        self.prevent_creation = Some(prevent);
        self
    }
}

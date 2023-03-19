use crate::{parsing::parse_response, pre::*};

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ConversationStarter {
    channel: Option<String>,
    prevent_creation: Option<bool>,
    return_im: Option<bool>,
    users: Vec<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct Conversation {
    pub channel: Option<String>,
    pub prevent_creation: Option<bool>,
    pub return_im: Option<bool>,
    pub users: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct Channel {
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

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct ConversationResponse {
    pub ok: bool,
    pub no_op: Option<bool>,
    pub already_open: Option<bool>,
    pub channel: Option<Channel>,
    pub error: Option<String>,
}

impl ConversationStarter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_user(mut self, user: &str) -> Self {
        self.users.push(user.to_string());
        self
    }

    pub fn channel(mut self, channel: &str) -> Self {
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

    pub fn build(self) -> Result<json::Value, Error> {
        let users = if self.users.is_empty() {
            None
        } else if self.users.len() > 1 {
            Some(
                self.users
                    .iter()
                    .map(|x| x.to_string() + ",")
                    .collect::<String>(),
            )
        } else {
            Some(self.users.get(0).unwrap().to_string())
        };

        match json::to_value(Conversation {
            channel: self.channel,
            prevent_creation: self.prevent_creation,
            return_im: self.return_im,
            users,
        }) {
            Ok(conversation) => Ok(conversation),
            Err(error) => Err(Error::Parsing(
                "Conversation".to_string(),
                error.to_string(),
            )),
        }
    }

    pub async fn start(self, token: &str) -> Result<ConversationResponse, Error> {
        let client = reqwest::Client::new();
        let json = self.build()?;
        let resp = match client
            .post("https://slack.com/api/conversations.open")
            .bearer_auth(token)
            .json(&json)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(error) => return Err(Error::Request(error)),
        };

        let conversation = parse_response::<ConversationResponse>(resp).await?;

        if let Some(error) = conversation.error {
            return Err(Error::Conversation(error));
        }

        Ok(conversation)
    }
}

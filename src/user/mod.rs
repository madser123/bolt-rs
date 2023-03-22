use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    Error, 
    parsing::{SlackResponse}, 
    SlackResult
};

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Profile {
    pub avatar_hash: Option<String>,
    pub status_text: Option<String>,
    pub status_emoji: Option<String>,
    pub real_name: Option<String>,
    pub display_name: Option<String>,
    pub real_name_normalized: Option<String>,
    pub email: Option<String>,
    pub image_24: Option<String>,
    pub image_32: Option<String>,
    pub image_48: Option<String>,
    pub image_72: Option<String>,
    pub image_192: Option<String>,
    pub image_512: Option<String>,
    pub team: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub team_id: String,
    pub name: String,
    pub deleted: bool,
    pub color: String,
    pub real_name: String,
    pub tz: String,
    pub tz_label: String,
    pub profile: Profile,
    pub is_admin: bool,
    pub is_owner: bool,
    pub is_primary_owner: bool,
    pub is_restricted: bool,
    pub is_ultra_restricted: bool,
    pub is_bot: bool,
    pub updated: i64,
    pub is_app_user: bool,
    pub has_2fa: bool,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserList {
    ok: bool,
    members: Option<Vec<User>>,
    cache_ts: Option<i64>,
    response_metadata: HashMap<String, String>,
    error: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Team {
    pub id: String,
    pub domain: String,
}

impl UserList {
    pub async fn new(token: &str) -> SlackResult<Self> {
        let client = reqwest::Client::new();
        let resp = client
            .post("https://slack.com/api/users.list")
            .bearer_auth(token)
            .send()
            .await?;

        let result: SlackResponse<Self> = SlackResponse::from_json(resp).await?;

        // Check for errors
        if !result.is_ok() {
            return Err(Error::User(result.error()))
        }

        if let Some(list) = result.value() {
            return Ok(list)
        }

        Err(Error::User("Recieved an OK response and an empty value?!".to_string()))
    }

    pub fn get_members(self) -> Option<Vec<User>> {
        self.members
    }
}

impl User {
    pub async fn from_id(token: &str, id: &str) {
        todo!()
    }

    pub async fn from_email(token: &str, email: &str) -> SlackResult<Self> {
        let client = reqwest::Client::new();
        let resp = client
            .get("https://slack.com/api/users.lookupByEmail")
            .bearer_auth(token)
            .send()
            .await?;

        let result: SlackResponse<Self> = SlackResponse::from_json(resp).await?;

        // Check for errors
        if !result.is_ok() {
            return Err(Error::User(result.error()))
        }

        if let Some(user) = result.value() {
            return Ok(user)
        }

        Err(Error::User("Recieved an OK response and an empty value?!".to_string()))
    }
}
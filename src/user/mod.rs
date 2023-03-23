use crate::pre::*;

use std::collections::HashMap;
use reqwest::multipart::Form;

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
    pub async fn new(token: &str) -> BoltResult<Self> {
        Request::post("users.list", token)
            .send()
            .await?
            .unpack()
    }

    pub fn get_members(self) -> Option<Vec<User>> {
        self.members
    }
}

impl User {
    pub async fn from_id(token: &str, id: &str) -> BoltResult<Self> {
        Request::get("users.info", token)
            .multipart(Form::new()
                .text("user", id.to_string())
            )
            .send()
            .await?
            .unpack()
    }

    pub async fn from_email(token: &str, email: &str) -> BoltResult<Self> {
        Request::get("users.lookupByEmail", token)
            .multipart(Form::new()
                .text("email", email.to_string()
            ))
            .send()
            .await?
            .unpack()
    }
}
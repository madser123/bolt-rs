use crate::pre::{skip_serializing_none, BoltResult, Deserialize, Request, Serialize};

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Profile {
    pub title: Option<String>,
    pub phone: Option<String>,
    pub skype: Option<String>,
    pub real_name: Option<String>,
    pub real_name_normalized: Option<String>,
    pub display_name: Option<String>,
    pub display_name_normalized: Option<String>,
    pub status_text: Option<String>,
    pub status_emoji: Option<String>,
    pub avatar_hash: Option<String>,
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
    pub color: Option<String>,
    pub real_name: Option<String>,
    // Maybe flatten tz_* as struct?
    pub tz: Option<String>,
    pub tz_label: Option<String>,
    pub tz_offset: Option<i64>,
    pub profile: Profile,
    // Flatten persmissions?
    pub is_admin: Option<bool>,
    pub is_owner: Option<bool>,
    pub is_primary_owner: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_ultra_restricted: Option<bool>,
    pub is_bot: bool,
    pub updated: i64,
    pub is_app_user: bool,
    pub has_2fa: Option<bool>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct List(Vec<User>);

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Team {
    pub id: String,
    pub domain: String,
}

impl List {
    /// Gets a complete list of users from slack
    ///
    /// # Errors
    ///
    /// An error will occur if the request fails to be sent or if slack reports any errors back.
    pub async fn new(token: &str) -> BoltResult<Self> {
        Request::post("users.list", token).send().await?.unpack()
    }

    /// Gets a specific user at an index
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&User> {
        self.0.get(index)
    }

    /// Returns the amount of users in the list
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Checks wether or not the list is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl std::iter::IntoIterator for List {
    type Item = User;
    type IntoIter = <Vec<User> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl User {
    /// Gets a user from a slack-id
    ///
    /// # Errors
    ///
    /// An error will occur if the request fails to be sent, or if slack reports any errors back.
    ///
    pub async fn from_id(token: &str, id: &str) -> BoltResult<Self> {
        Request::get(&format!("users.info?user={id}"), token)
            .send()
            .await?
            .unpack()
    }

    /// Gets a user from an email
    ///
    /// # Errors
    ///
    /// An error will occur if the request fails to be sent, or if slack reports any errors back.
    ///
    pub async fn from_email(token: &str, email: &str) -> BoltResult<Self> {
        Request::get(&format!("users.lookupByEmail?email={email}"), token)
            .send()
            .await?
            .unpack()
    }
}

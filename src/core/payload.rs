use super::*;
use comp::{Text, Any};
use view::View;
use user::Team;
use crate::app::{Interaction, Error as AppError};

// Types

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseUrl {
    block_id: Option<String>,
    action_id: Option<String>,
    channel_id: Option<String>,
    response_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    pub r#type: String,
    pub user: String,
    pub ts: String,
    pub text: String,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Channel {
    pub id: String,
    pub name: Option<String>,
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Action {
    pub action_id: String,
    pub block_id: Option<String>,
    pub text: Text<Any>,
    pub value: Option<String>,
    pub r#type: String,
    pub action_ts: String,
}

// Payloads

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlockAction {
    pub r#type: String,
    pub trigger_id: String,
    pub response_url: String,
    pub user: ResponseUser,
    pub message: Option<Message>,
    pub view: Option<View>,
    pub actions: Vec<Action>,
    pub hash: String,
    pub state: Option<state::State>,
}
impl Interaction for BlockAction {
    fn identifier(&self) -> String {
        self.trigger_id.clone()
    }

    fn error(message: String) -> crate::app::Error {
        AppError::BlockAction(message)
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MessageAction {
    /// The type of interaction used. This will always be "message_shortcut" for this type of payload.
    pub r#type: String,

    /// The shortcut/interactions name/id.
    pub callback_id: String,

    /// An id created for the interaction itself. This can be used to open modals.
    pub trigger_id: String,

    pub response_url: String,

    /// The user who interacted.
    pub user: ResponseUser,

    pub message: Message,

    pub channel: Channel,

    /// The team (Workspace) the interaction originates from.
    pub team: Team,
    // /// Each request sends the bots `verification token` for verification | Deprecated - Use signed secrets instead.
    // pub token: String,
}
impl Interaction for MessageAction {
    fn identifier(&self) -> String {
        self.callback_id.clone()
    }

    fn error(message: String) -> crate::app::Error {
        AppError::MessageAction(message)
    }
}

/// A payload sent from slack for app-shortcuts.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Shortcut {
    /// The type of interaction used. This will always be "shortcut" for this type of payload.
    pub r#type: String,

    /// The shortcut/interactions name/id.
    pub callback_id: String,

    /// An id created for the interaction itself. This can be used to open modals.
    pub trigger_id: String,

    // /// Each request sends the bots `verification token` for verification | Deprecated - Use signed secrets instead.
    // pub token: String,
    /// A timestamp for when the action was executed.
    pub action_ts: String,

    /// The team (Workspace) the interaction originates from.
    pub team: Team,

    /// The user who interacted.
    pub user: ResponseUser,
}
impl Interaction for Shortcut {
    fn identifier(&self) -> String {
        self.callback_id.clone()
    }

    fn error(message: String) -> crate::app::Error {
        AppError::Shortcut(message)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseUser {
    pub id: String,
    pub username: String,
    pub team_id: String,
}

/// A payload sent from slack for view-submissions.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ViewSubmission {
    pub r#type: String,

    pub team: Team,

    pub user: ResponseUser,

    pub view: View<ModalResponse>,

    pub hash: Option<String>,

    pub response_urls: Vec<ResponseUrl>,
}
impl Interaction for ViewSubmission {
    fn identifier(&self) -> String {
        todo!()
    }

    fn error(message: String) -> crate::app::Error {
        AppError::ViewSubmission(message)
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ViewClosed {
    pub r#type: String,
    pub team: Team,
    pub user: ResponseUser,
    pub view: View<ModalResponse>,
    pub is_cleared: bool,
}
impl Interaction for ViewClosed {
    fn identifier(&self) -> String {
        todo!()
    }

    fn error(message: String) -> crate::app::Error {
        AppError::ViewClosed(message)
    }
}

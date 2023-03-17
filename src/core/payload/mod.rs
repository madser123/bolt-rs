use super::*;
use crate::{
    user::{User, Team},
    view::{View, ModalResponse},
    comp::{Text, Any},
};

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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ViewClosed {
    pub r#type: String,
    pub team: Team,
    pub user: ResponseUser,
    pub view: View<ModalResponse>,
    pub is_cleared: bool,
}
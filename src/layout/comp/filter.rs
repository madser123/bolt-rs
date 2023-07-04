use super::{skip_serializing_none, Composition, Debug, Deserialize, Serialize};

/// A filter inclusion-parameter
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Inclusion {
    /// Conversations with the calling user/app
    Im,

    // Group-conversations with the calling user/app
    MpIm,

    /// Private channels
    Private,

    /// Public channels
    Public,
}

/// A composition-block of type `filter`
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Filter {
    include: Vec<Inclusion>,
    exclude_external_shared_channels: bool,
    exclude_bot_users: bool,
}
impl Composition for Filter {}
impl Filter {
    /// Creates a new [`Filter`] composition-block
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Includes the specified types of channels/conversations
    #[must_use]
    pub fn include(mut self, mut include: Vec<Inclusion>) -> Self {
        self.include.append(&mut include);
        self
    }

    /// Excludes external channels
    #[must_use]
    pub const fn exclude_external_shared_channels(mut self) -> Self {
        self.exclude_external_shared_channels = true;
        self
    }

    /// Excludes conversations with bot users
    #[must_use]
    pub const fn exclude_bot_users(mut self) -> Self {
        self.exclude_bot_users = true;
        self
    }
}

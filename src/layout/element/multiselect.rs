use super::{
    option, skip_serializing_none, Build, Confirmation, ConversationList, Debug, Deserialize,
    Element, ExternalData, Filter, InputElement, Menu, Plain, PublicChannels, SectionElement,
    Serialize, StaticOptions, Text, UserList,
};

/// Represents an element of type `multiselect`
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct MultiSelect<T: Menu = StaticOptions> {
    #[serde(skip_serializing)]
    t: std::marker::PhantomData<T>,

    r#type: String,
    action_id: String,
    confirm: Option<Confirmation>,
    max_selected_items: Option<i64>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text<Plain>>,

    options: Option<Vec<option::Object<Plain>>>,
    option_groups: Option<Vec<option::Group>>,

    initial_options: Option<Vec<option::Object<Plain>>>,

    min_query_length: Option<i64>,

    initial_users: Option<Vec<String>>,

    initial_conversations: Option<Vec<String>>,
    default_to_current_conversation: Option<bool>,
    filter: Option<Filter>,

    initial_channels: Option<Vec<String>>,
}
impl MultiSelect {
    /// Creates a new [`MultiSelect`] element with static options.
    #[must_use]
    pub fn static_options(action_id: &str, options: Vec<option::Object<Plain>>) -> Self {
        Self {
            r#type: "multi_static_select".to_string(),
            action_id: action_id.to_string(),
            options: Some(options),
            ..Default::default()
        }
    }

    /// Creates a new [`MultiSelect`] element with external data (options-load url).
    #[must_use]
    pub fn external_data(action_id: &str) -> MultiSelect<ExternalData> {
        MultiSelect::<ExternalData> {
            r#type: "multi_external_select".to_string(),
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    /// Creates a new [`MultiSelect`] element with a user-list.
    #[must_use]
    pub fn user_list(action_id: &str) -> MultiSelect<UserList> {
        MultiSelect::<UserList> {
            r#type: "multi_users_select".to_string(),
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    /// Creates a new [`MultiSelect`] element with a list of conversations.
    #[must_use]
    pub fn conversation_list(action_id: &str) -> MultiSelect<ConversationList> {
        MultiSelect::<ConversationList> {
            r#type: "multi_conversations_select".to_string(),
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    /// Creates a new [`MultiSelect`] element with a list of public channels.
    #[must_use]
    pub fn public_channels(action_id: &str) -> MultiSelect<PublicChannels> {
        MultiSelect::<PublicChannels> {
            r#type: "multi_channels_select".to_string(),
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }
}
impl<T: Menu> MultiSelect<T>
where
    Self: Element,
{
    /// Adds a confirmation dialogue to the field
    #[must_use]
    pub fn confirm(mut self, confirm: Confirmation) -> Self {
        self.confirm = Some(confirm);
        self
    }

    /// Sets the max amount of options selected
    #[must_use]
    pub const fn max_selected(mut self, max: i64) -> Self {
        self.max_selected_items = Some(max);
        self
    }

    /// Forces the element to be focused upon load
    #[must_use]
    pub const fn focus_on_load(mut self) -> Self {
        self.focus_on_load = Some(true);
        self
    }

    /// Adds placeholder-text to the field
    #[must_use]
    pub fn placeholder(mut self, text: Text<Plain>) -> Self {
        self.placeholder = Some(text);
        self
    }
}
impl MultiSelect<StaticOptions> {
    /// Provide groupings of options
    #[must_use]
    pub fn option_groups(mut self, groups: Vec<option::Group>) -> Self {
        self.option_groups = Some(groups);
        self
    }

    /// Sets the initial options to be selected upon load
    #[must_use]
    pub fn initial_options(mut self, options: Vec<option::Object<Plain>>) -> Self {
        self.initial_options = Some(options);
        self
    }
}
impl MultiSelect<ExternalData> {
    /// Sets the minimum query-length for the field
    #[must_use]
    pub const fn min_query_length(mut self, length: i64) -> Self {
        self.min_query_length = Some(length);
        self
    }

    /// Sets the initial options to be selected upon load
    #[must_use]
    pub fn initial_options(mut self, options: Vec<option::Object<Plain>>) -> Self {
        self.initial_options = Some(options);
        self
    }
}
impl MultiSelect<UserList> {
    /// Sets the initial users to be selected upon load
    #[must_use]
    pub fn initial_users(mut self, users: Vec<String>) -> Self {
        self.initial_users = Some(users);
        self
    }
}
impl MultiSelect<ConversationList> {
    /// Sets the initial conversations to be selected upon load
    #[must_use]
    pub fn initial_conversations(mut self, conversations: Vec<String>) -> Self {
        self.initial_conversations = Some(conversations);
        self
    }

    /// Defaults to current conversation as selected upon load
    #[must_use]
    pub const fn default_to_current(mut self) -> Self {
        self.default_to_current_conversation = Some(true);
        self
    }

    /// Add a conversation-filter to the list
    #[must_use]
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
}
impl MultiSelect<PublicChannels> {
    /// Sets the initial channels to be selected upon load
    #[must_use]
    pub fn initial_channels(mut self, channels: Vec<String>) -> Self {
        self.initial_channels = Some(channels);
        self
    }
}
impl SectionElement for MultiSelect<StaticOptions> {}
impl InputElement for MultiSelect<StaticOptions> {}
impl Element for MultiSelect<StaticOptions> {}
impl Build for MultiSelect<StaticOptions> {
    fn get_type(&self) -> String {
        "multi_static_select".to_string()
    }
}
impl SectionElement for MultiSelect<ExternalData> {}
impl InputElement for MultiSelect<ExternalData> {}
impl Element for MultiSelect<ExternalData> {}
impl Build for MultiSelect<ExternalData> {
    fn get_type(&self) -> String {
        "multi_external_select".to_string()
    }
}
impl SectionElement for MultiSelect<UserList> {}
impl InputElement for MultiSelect<UserList> {}
impl Element for MultiSelect<UserList> {}
impl Build for MultiSelect<UserList> {
    fn get_type(&self) -> String {
        "multi_users_select".to_string()
    }
}
impl SectionElement for MultiSelect<ConversationList> {}
impl InputElement for MultiSelect<ConversationList> {}
impl Element for MultiSelect<ConversationList> {}
impl Build for MultiSelect<ConversationList> {
    fn get_type(&self) -> String {
        "multi_conversations_select".to_string()
    }
}
impl SectionElement for MultiSelect<PublicChannels> {}
impl InputElement for MultiSelect<PublicChannels> {}
impl Element for MultiSelect<PublicChannels> {}
impl Build for MultiSelect<PublicChannels> {
    fn get_type(&self) -> String {
        "multi_channels_select".to_string()
    }
}

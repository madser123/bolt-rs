use super::{
    option, skip_serializing_none, ActionsElement, Build, Confirmation, ConversationList, Debug,
    Deserialize, Element, ExternalData, Filter, InputElement, Menu, Plain, PublicChannels,
    SectionElement, Serialize, StaticOptions, Text, UserList,
};

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Select<M: Menu = StaticOptions> {
    #[serde(skip_serializing)]
    t: std::marker::PhantomData<M>,

    r#type: String,
    action_id: String,
    confirm: Option<Confirmation>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text<Plain>>,

    options: Option<Vec<option::Object<Plain>>>,
    option_groups: Option<Vec<option::Group>>,

    initial_option: Option<option::Object<Plain>>,

    min_query_length: Option<i64>,

    initial_user: Option<String>,

    initial_conversation: Option<String>,
    default_to_current_conversation: Option<bool>,
    response_url_enabled: Option<bool>,
    filter: Option<Filter>,

    initial_channel: Option<String>,
}
impl Select {
    /// Creates a new [`Select`] element with static options.
    #[must_use]
    pub fn static_options(action_id: &str, options: Vec<option::Object<Plain>>) -> Self {
        Self {
            r#type: "static_select".to_string(),
            action_id: action_id.to_string(),
            options: Some(options),
            ..Default::default()
        }
    }

    /// Creates a new [`Select`] element with external-data
    #[must_use]
    pub fn external_data(action_id: &str) -> Self {
        Self {
            r#type: "external_select".to_string(),
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    /// Creates a new [Select] element with a user-list.
    #[must_use]
    pub fn user_list(action_id: &str) -> Self {
        Self {
            r#type: "users_select".to_string(),
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    /// Creates a new [Select] element with a conversation-list.
    #[must_use]
    pub fn conversation_list(action_id: &str) -> Self {
        Self {
            r#type: "conversations_select".to_string(),
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    /// Creates a new [Select] element with a public-channels list.
    #[must_use]
    pub fn public_channels(action_id: &str) -> Self {
        Self {
            r#type: "channels_select".to_string(),
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }
}
impl<M: Menu> Select<M>
where
    Self: Element,
{
    /// Applies confirmation to the element
    #[must_use]
    pub fn confirm(mut self, confirm: Confirmation) -> Self {
        self.confirm = Some(confirm);
        self
    }

    /// Sets the element to be focused on load.
    #[must_use]
    pub const fn focus_on_load(mut self, focus: bool) -> Self {
        self.focus_on_load = Some(focus);
        self
    }

    /// Sets the placeholder-text for the element.
    #[must_use]
    pub fn placeholder(mut self, text: Text<Plain>) -> Self {
        self.placeholder = Some(text);
        self
    }
}
impl Select<StaticOptions> {
    /// Groups the options into sections that the user can see
    #[must_use]
    pub fn option_groups(mut self, groups: Vec<option::Group>) -> Self {
        self.option_groups = Some(groups);
        self
    }

    /// Sets the initial option selected
    #[must_use]
    pub fn initial_option(mut self, option: option::Object<Plain>) -> Self {
        self.initial_option = Some(option);
        self
    }
}
impl Select<ExternalData> {
    /// Sets the minimum query-length allowed
    #[must_use]
    pub const fn min_query_length(mut self, length: i64) -> Self {
        self.min_query_length = Some(length);
        self
    }

    /// Sets the initial option selected
    #[must_use]
    pub fn initial_option(mut self, option: option::Object<Plain>) -> Self {
        self.initial_option = Some(option);
        self
    }
}
impl Select<UserList> {
    /// Sets the initial user selected
    #[must_use]
    pub fn initial_user(mut self, user: String) -> Self {
        self.initial_user = Some(user);
        self
    }
}
impl Select<ConversationList> {
    /// Sets the initial conversation selected
    #[must_use]
    pub fn initial_conversation(mut self, conversation: String) -> Self {
        self.initial_conversation = Some(conversation);
        self
    }

    /// Sets the initial conversation selected to be the current one.
    #[must_use]
    pub const fn default_to_current(mut self) -> Self {
        self.default_to_current_conversation = Some(true);
        self
    }

    /// Applies a filter to the conversation-list.
    #[must_use]
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Enables a response-url for the element
    #[must_use]
    pub const fn enable_response_url(mut self) -> Self {
        self.response_url_enabled = Some(true);
        self
    }
}
impl Select<PublicChannels> {
    /// Sets the initial channel selected.
    #[must_use]
    pub fn initial_channel(mut self, channel: String) -> Self {
        self.initial_channel = Some(channel);
        self
    }

    /// Enables a response-url for the element
    #[must_use]
    pub const fn enable_response_url(mut self) -> Self {
        self.response_url_enabled = Some(true);
        self
    }
}
impl SectionElement for Select<StaticOptions> {}
impl ActionsElement for Select<StaticOptions> {}
impl InputElement for Select<StaticOptions> {}
impl Element for Select<StaticOptions> {}
impl Build for Select<StaticOptions> {
    fn get_type(&self) -> String {
        "static_select".to_string()
    }
}
impl SectionElement for Select<ExternalData> {}
impl ActionsElement for Select<ExternalData> {}
impl InputElement for Select<ExternalData> {}
impl Element for Select<ExternalData> {}
impl Build for Select<ExternalData> {
    fn get_type(&self) -> String {
        "external_select".to_string()
    }
}
impl SectionElement for Select<UserList> {}
impl ActionsElement for Select<UserList> {}
impl InputElement for Select<UserList> {}
impl Element for Select<UserList> {}
impl Build for Select<UserList> {
    fn get_type(&self) -> String {
        "users_select".to_string()
    }
}
impl SectionElement for Select<ConversationList> {}
impl ActionsElement for Select<ConversationList> {}
impl InputElement for Select<ConversationList> {}
impl Element for Select<ConversationList> {}
impl Build for Select<ConversationList> {
    fn get_type(&self) -> String {
        "conversations_select".to_string()
    }
}
impl SectionElement for Select<PublicChannels> {}
impl ActionsElement for Select<PublicChannels> {}
impl InputElement for Select<PublicChannels> {}
impl Element for Select<PublicChannels> {}
impl Build for Select<PublicChannels> {
    fn get_type(&self) -> String {
        "channels_select".to_string()
    }
}

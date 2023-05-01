use super::*;

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

    options: Option<Vec<OptionObject<Plain>>>,
    option_groups: Option<Vec<OptionGroup>>,

    initial_options: Option<Vec<OptionObject<Plain>>>,

    min_query_length: Option<i64>,

    initial_users: Option<Vec<String>>,

    initial_conversations: Option<Vec<String>>,
    default_to_current_conversation: Option<bool>,
    filter: Option<Filter>,

    initial_channels: Option<Vec<String>>,
}
impl MultiSelect {
    /// Creates a new [MultiSelect] element with static options.
    pub fn static_options(
        action_id: &str,
        options: Vec<OptionObject<Plain>>,
    ) -> MultiSelect<StaticOptions> {
        MultiSelect::<StaticOptions> {
            r#type: "multi_static_select".to_string(),
            action_id: action_id.to_string(),
            options: Some(options),
            ..Default::default()
        }
    }

    /// Creates a new [MultiSelect] element with external data (options-load url).
    pub fn external_data(action_id: &str) -> MultiSelect<ExternalData> {
        MultiSelect::<ExternalData> {
            r#type: "multi_external_select".to_string(),
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    /// Creates a new [MultiSelect] element with a user-list.
    pub fn user_list(action_id: &str) -> MultiSelect<UserList> {
        MultiSelect::<UserList> {
            r#type: "multi_users_select".to_string(),
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    /// Creates a new [MultiSelect] element with a list of conversations.
    pub fn conversation_list(action_id: &str) -> MultiSelect<ConversationList> {
        MultiSelect::<ConversationList> {
            r#type: "multi_conversations_select".to_string(),
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    /// Creates a new [MultiSelect] element with a list of public channels.
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
    pub fn confirm(mut self, confirm: Confirmation) -> Self {
        self.confirm = Some(confirm);
        self
    }

    /// Sets the max amount of options selected
    pub fn max_selected(mut self, max: i64) -> Self {
        self.max_selected_items = Some(max);
        self
    }

    /// Forces the element to be focused upon load
    pub fn focus_on_load(mut self) -> Self {
        self.focus_on_load = Some(true);
        self
    }

    /// Adds placeholder-text to the field
    pub fn placeholder(mut self, text: Text<Plain>) -> Self {
        self.placeholder = Some(text);
        self
    }
}
impl MultiSelect<StaticOptions> {
    /// Provide groupings of options
    pub fn option_groups(mut self, groups: Vec<OptionGroup>) -> Self {
        self.option_groups = Some(groups);
        self
    }

    /// Sets the initial options to be selected upon load
    pub fn initial_options(mut self, options: Vec<OptionObject<Plain>>) -> Self {
        self.initial_options = Some(options);
        self
    }
}
impl MultiSelect<ExternalData> {
    /// Sets the minimum query-length for the field
    pub fn min_query_length(mut self, length: i64) -> Self {
        self.min_query_length = Some(length);
        self
    }

    /// Sets the initial options to be selected upon load
    pub fn initial_options(mut self, options: Vec<OptionObject<Plain>>) -> Self {
        self.initial_options = Some(options);
        self
    }
}
impl MultiSelect<UserList> {
    /// Sets the initial users to be selected upon load
    pub fn initial_users(mut self, users: Vec<String>) -> Self {
        self.initial_users = Some(users);
        self
    }
}
impl MultiSelect<ConversationList> {
    /// Sets the initial conversations to be selected upon load
    pub fn initial_conversations(mut self, conversations: Vec<String>) -> Self {
        self.initial_conversations = Some(conversations);
        self
    }

    /// Defaults to current conversation as selected upon load
    pub fn default_to_current(mut self) -> Self {
        self.default_to_current_conversation = Some(true);
        self
    }

    /// Add a conversation-filter to the list
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
}
impl MultiSelect<PublicChannels> {
    /// Sets the initial channels to be selected upon load
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

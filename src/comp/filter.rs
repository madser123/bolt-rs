use super::*;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Filter {
    include: Vec<String>,
    exclude_external_shared_channels: bool,
    exclude_bot_users: bool,
}
impl Composition for Filter {}
impl Filter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn include(mut self, include: &str) -> Self {
        self.include.push(include.to_string());
        self
    }

    pub fn include_many(mut self, include: Vec<&str>) -> Self {
        self.include.append(&mut include.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn exclude_external_shared_channels(mut self) -> Self {
        self.exclude_external_shared_channels = true;
        self
    }

    pub fn exclude_bot_users(mut self) -> Self {
        self.exclude_bot_users = true;
        self
    }
}
impl Build for Filter {
    fn get_type(&self) -> String {
        "filter".to_string()
    }
}
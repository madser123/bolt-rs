use super::{parsing, Debug, Deserialize, Serialize, View};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Controller<T: parsing::SerializeDefaultPhantomData> {
    view: View<T>,
    trigger_id: Option<String>,
    external_id: Option<String>,
}

impl<T: Serialize + parsing::SerializeDefaultPhantomData> Controller<T> {
    /// Creates a new [`Controller`] from a trigger
    #[must_use]
    pub fn trigger(trigger: &str, view: View<T>) -> Self {
        Self {
            view,
            trigger_id: Some(trigger.to_string()),
            external_id: None,
        }
    }

    /// Creates a new [`Controller`] to update a view
    #[must_use]
    pub fn update(view: View<T>) -> Self {
        Self {
            external_id: view.external_id.clone(),
            view,
            trigger_id: None,
        }
    }
}

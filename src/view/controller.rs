use super::*;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Controller<T: Serialize + parsing::SerializeDefaultPhantomData> {
    view: View<T>,
    trigger_id: Option<String>,
    external_id: Option<String>,
}

impl<T: Serialize + parsing::SerializeDefaultPhantomData> Controller<T> {
    pub fn trigger(trigger: &str, view: View<T>) -> Self {
        Controller {
            view,
            trigger_id: Some(trigger.to_string()),
            external_id: None,
        }
    }

    pub fn update(view: View<T>) -> Self {
        Controller {
            external_id: view.external_id.clone(),
            view,
            trigger_id: None,
        }
    }
}
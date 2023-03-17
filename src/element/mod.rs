use crate::{Error, core::Build, comp::{Text, Plain, Any, Style, Confirmation, DispatchActionConfig, OptionGroup, OptionObject, Filter}, block::{Input, Section, Actions}};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use serde_json as json;

mod button;
mod checkboxes;
mod datepicker;
mod datetimepicker;
mod email;
mod image;
mod multiselect;
mod number;
mod overflow;
mod plaintext;
mod radiobuttons;
mod select;
mod timepicker;
mod url;

pub use self::button::Button;
pub use self::checkboxes::Checkboxes;
pub use self::datepicker::DatePicker;
pub use self::datetimepicker::DatetimePicker;
pub use self::email::Email;
pub use self::image::Image;
pub use self::multiselect::MultiSelect;
pub use self::number::Number;
pub use self::overflow::Overflow;
pub use self::plaintext::PlainTextInput;
pub use self::radiobuttons::RadioButtons;
pub use self::select::Select;
pub use self::timepicker::TimePicker;
pub use self::url::UrlInput;

pub trait Element: Build  {    
    fn parse_action_id(&self, id: &str) -> Result<(), Error> {
        if id.is_empty() {
            return Err(Error::Element(self.get_type(), "No action_id defined!".to_string()))
        } else if id.len() > 255 {
            return Err(Error::Element(self.get_type(), "Action_id must be under 255 characters!".to_string()))
        }
        Ok(())
    }
}

pub trait AsElements {
    /// Turns `self` into a list of `Elements`
    fn as_elements(&self) -> Result<Elements, Error>;
}
pub trait AsElement {
    /// Turns `self` into an `Element` of type `T`
    fn as_element<T: Element>(&self) -> Result<T, Error>;
}

pub trait SectionElement: Element
where
    Self: Sized,
{
    fn into_section_accessory(self) -> Result<Section, Error> {
        Section::new().accessory(self)
    }
}

pub trait ActionsElement: Element
where
    Self: Sized,
{
    fn into_actions(self) -> Result<Actions, Error> {
        Actions::new().elements(vec![self])
    }
}
pub trait InputElement: Element
where
    Self: Sized,
{
    fn into_input(self, label: Text<Plain>) -> Input<Self> {
        Input::new(self, label)
    }
}
pub trait ContextElement: Element {}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Elements(Vec<json::Value>);

impl Elements {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, element: &impl Element) -> Result<(), Error> {
        self.0.push(element.build()?);
        Ok(())
    }

    pub fn append(&mut self, elements: &mut Vec<impl Element>) -> Result<(), Error> {
        for e in elements {
            self.push(e)?;
        }
        Ok(())
    }

    pub fn json(self) -> Vec<json::Value> {
        self.0
    }
}

pub trait Menu {}

#[derive(Debug, Default)]
pub struct StaticOptions {}
impl Menu for StaticOptions {}

#[derive(Debug, Default)]
pub struct ExternalData {}
impl Menu for ExternalData {}

#[derive(Debug, Default)]
pub struct UserList {}
impl Menu for UserList {}

#[derive(Debug, Default)]
pub struct ConversationList {}
impl Menu for ConversationList {}

#[derive(Debug, Default)]
pub struct PublicChannels {}
impl Menu for PublicChannels {}

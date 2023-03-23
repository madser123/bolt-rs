use super::*;
use block::{Actions, Input, Section};
use comp::*;

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

pub trait Element: Build {}

pub trait AsElements {
    /// Turns `self` into a list of `Elements`
    fn as_elements(&self) -> BoltResult<Elements>;
}
pub trait AsElement {
    /// Turns `self` into an `Element` of type `T`
    fn as_element<T: Element>(&self) -> BoltResult<T>;
}

pub trait SectionElement: Element
where
    Self: Sized,
{
    fn into_section_as_accessory(self) -> BoltResult<Section> {
        Section::new().accessory(self)
    }
}

pub trait ActionsElement: Element
where
    Self: Sized,
{
    fn into_actions(self) -> BoltResult<Actions> {
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

    pub fn push(&mut self, element: &impl Element) -> BoltResult<Null> {
        self.0.push(element.build()?);
        Ok(Null::Null)
    }

    pub fn append(&mut self, elements: &mut Vec<impl Element>) -> BoltResult<Null> {
        for e in elements {
            self.push(e)?;
        }
        Ok(Null::Null)
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

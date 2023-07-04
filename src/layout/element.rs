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

/// Converts any type into a list of elements
pub trait AsElements {
    /// Turns `self` into a list of `Elements`
    fn as_elements(&self) -> BoltResult<Elements>;
}

/// Converts any type into a single element
pub trait AsElement<E> {
    /// Turns `self` into an `Element` of type `T`
    fn as_element(&self) -> BoltResult<E>;
}

/// Convert elements into Section accessories
pub trait SectionElement: Element
where
    Self: Sized,
{
    fn into_section_as_accessory(self) -> BoltResult<Section> {
        Section::new().accessory(self)
    }
}

/// Convert elements into action-elements
pub trait ActionsElement: Element
where
    Self: Sized,
{
    fn into_actions(self) -> BoltResult<Actions> {
        Actions::new().elements(vec![self])
    }
}

/// Convert elements into inputs
pub trait InputElement: Element
where
    Self: Sized,
{
    fn into_input(self, label: Text<Plain>) -> Input<Self> {
        Input::new(self, label)
    }
}

/// Needed when using elements in contexts
pub trait ContextElement: Element {}

/// A collection of elements
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Elements(Vec<json::Value>);

impl Elements {
    /// Creates a new empty list of elements
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes an element onto the list
    pub fn push(&mut self, element: &impl Element) -> BoltResult<()> {
        self.0.push(element.build()?);
        Ok(())
    }

    /// Appends a list of elements to the list
    pub fn append(&mut self, elements: &mut Vec<impl Element>) -> BoltResult<()> {
        for e in elements {
            self.push(e)?;
        }
        Ok(())
    }

    /// Output as a list of json values
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

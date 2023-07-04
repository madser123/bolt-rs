use super::{
    block::{Actions, Input, Section},
    comp::{option, Any, Confirmation, DispatchActionConfig, Filter, Plain, Text},
    json, skip_serializing_none, BoltResult, Build, Debug, Deserialize, Serialize, Style,
};

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
pub use self::url::Url;

pub trait Element: Build {}

/// Converts any type into a list of elements
pub trait AsElements {
    /// Turns `self` into a list of `Elements`
    ///
    /// # Errors
    ///
    /// An error should occur if the elements fails serializing.
    ///
    fn as_elements(&self) -> BoltResult<Elements>;
}

/// Converts any type into a single element
#[allow(clippy::module_name_repetitions)]
pub trait AsElement<E> {
    /// Turns `self` into an `Element` of type `T`
    ///
    /// # Errors
    ///
    /// An error should occur if the element fails serializing.
    ///
    fn as_element(&self) -> BoltResult<E>;
}

/// Convert elements into Section accessories
#[allow(clippy::module_name_repetitions)]
pub trait SectionElement: Element
where
    Self: Sized,
{
    /// Converts this element into a section-accessory.
    ///
    /// # Errors
    ///
    /// An error will occur if the element fails to serialize.
    ///
    fn into_section_as_accessory(self) -> BoltResult<Section> {
        Section::new().accessory(&self)
    }
}

/// Convert elements into action-elements
#[allow(clippy::module_name_repetitions)]
pub trait ActionsElement: Element
where
    Self: Sized,
{
    /// Converts this element into an action-block, with this in the elements.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    fn into_actions(self) -> BoltResult<Actions> {
        Actions::new().elements(vec![self])
    }
}

/// Convert elements into inputs
#[allow(clippy::module_name_repetitions)]
pub trait InputElement: Element
where
    Self: Sized,
{
    fn into_input(self, label: Text<Plain>) -> Input<Self> {
        Input::new(self, label)
    }
}

/// Needed when using elements in contexts
#[allow(clippy::module_name_repetitions)]
pub trait ContextElement: Element {}

/// A collection of elements
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Elements(Vec<json::Value>);

impl Elements {
    /// Creates a new empty list of elements
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes an element onto the list
    ///
    /// # Errors
    ///
    /// An error will occur if the supplied element fails serializing.
    ///
    pub fn push(&mut self, element: &impl Element) -> BoltResult<()> {
        self.0.push(element.build()?);
        Ok(())
    }

    /// Appends a list of elements to the list
    ///
    /// # Errors
    ///
    /// An error will occur if one or more of the supplied elements fails serializing.
    ///
    pub fn append(&mut self, elements: &mut Vec<impl Element>) -> BoltResult<()> {
        for e in elements {
            self.push(e)?;
        }
        Ok(())
    }

    /// Output as a list of json values
    #[must_use]
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

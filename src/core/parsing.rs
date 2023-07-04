use crate::pre::Deserialize;
use serde::{de::IgnoredAny, Deserializer};

/// Used internally to help serde deserialize structs with phantomdata.
///
/// # Errors
///
/// Errors will occur if deserializing fails.
///
pub fn default_phantomdata<'de, D, T>(
    deserializer: D,
) -> Result<std::marker::PhantomData<T>, D::Error>
where
    D: Deserializer<'de>,
    T: SerializeDefaultPhantomData,
{
    // Ignore the data in the input.
    IgnoredAny::deserialize(deserializer)?;
    Ok(T::new())
}

/// Trait defining behaviour for structs that wish to use `default_phantomdata()` for deserializing.
pub trait SerializeDefaultPhantomData {
    #[must_use]
    fn new() -> std::marker::PhantomData<Self> {
        std::marker::PhantomData::<Self>
    }
}

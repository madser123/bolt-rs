use crate::pre::*;
use serde::{Deserializer, de::IgnoredAny};

/// Used internally to help serde deserialize structs with phantomdata.
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
    fn new() -> std::marker::PhantomData<Self> {
        std::marker::PhantomData::<Self>
    }
}

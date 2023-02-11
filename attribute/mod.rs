use std::error::Error;

pub mod extension;
pub mod frequency;
pub mod location;
pub mod modified;
pub mod priority;

pub trait AsAttribute {
    type Error: Error;

    fn parse(raw: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait AsUnderlying<T> {
    fn as_underlying(&self) -> T;
}

use std::error::Error;

mod extension;
mod frequency;
mod location;
mod modified;
mod priority;

pub use self::extension::*;
pub use self::frequency::*;
pub use self::location::*;
pub use self::modified::*;
pub use self::priority::*;

pub trait AsAttribute {
    type Error: Error;

    fn parse(raw: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait AsUnderlying<T> {
    fn as_underlying(&self) -> T;
}

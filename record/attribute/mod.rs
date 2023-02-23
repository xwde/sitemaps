#[cfg(any(feature = "txt", feature = "xml"))]
mod location;
#[cfg(any(feature = "txt", feature = "xml"))]
pub use location::*;

#[cfg(feature = "xml")]
mod frequency;
#[cfg(feature = "xml")]
mod modified;
#[cfg(feature = "xml")]
mod priority;

#[cfg(feature = "xml")]
pub use frequency::*;
#[cfg(feature = "xml")]
pub use modified::*;
#[cfg(feature = "xml")]
pub use priority::*;

use std::error::Error;

pub trait Attribute<T>: Sized {
    type Error: Error;

    fn parse(attribute: &str) -> Result<Self, Self::Error>;
    fn build(&self) -> String;
    fn underlying(&self) -> T;
}

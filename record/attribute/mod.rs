mod location;
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
use std::fmt::Display;

pub trait Attribute<T>: Sized + Display {
    type Error: Error;

    fn parse(attribute: &str) -> Result<Self, Self::Error>;
    fn underlying(&self) -> T;
}

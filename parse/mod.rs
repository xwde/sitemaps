#[cfg(feature = "txt")]
mod txt;
#[cfg(feature = "txt")]
pub use txt::*;

#[cfg(feature = "xml")]
mod xml;
#[cfg(feature = "xml")]
pub use xml::*;

use std::error::Error;
use std::io::Read;

use crate::Record;

pub trait Parser<R: Read, D: Record>: Sized {
    type Error: Error;

    // fn parse(reader: R) -> Result<(), Self::Error> {
    //     todo!()
    // }

    fn initialize(reader: R) -> Result<Self, Self::Error>;
    fn next(&mut self) -> Result<Option<D>, Self::Error>;
    fn finalize(self) -> Result<R, Self::Error>;
}

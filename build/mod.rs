#[cfg(feature = "txt")]
mod txt;
#[cfg(feature = "txt")]
pub use txt::*;

#[cfg(feature = "xml")]
mod xml;
#[cfg(feature = "xml")]
pub use xml::*;

use std::error::Error;
use std::io::Write;

use crate::record::Record;

pub trait Builder<W: Write, D: Record>: Sized {
    type Error: Error;

    fn build(writer: W, records: impl Iterator<Item = D>) -> Result<W, Self::Error> {
        let mut builder = Self::initialize(writer)?;
        for record in records {
            builder.next(record)?;
        }

        builder.finalize()
    }

    fn initialize(writer: W) -> Result<Self, Self::Error>;
    fn next(&mut self, record: D) -> Result<(), Self::Error>;
    fn finalize(self) -> Result<W, Self::Error>;
}

pub trait StringBuilder<D: Record>: Builder<Vec<u8>, D> {
    fn build_string(container: impl Iterator<Item = D>) -> Result<String, Self::Error>;
}

impl<D: Record, T> StringBuilder<D> for T
where
    T: Builder<Vec<u8>, D>,
{
    fn build_string(records: impl Iterator<Item = D>) -> Result<String, Self::Error> {
        let buffer = Vec::new();
        let buffer = Self::build(buffer, records)?;
        Ok(String::from_utf8_lossy(buffer.as_slice()).to_string())
    }
}

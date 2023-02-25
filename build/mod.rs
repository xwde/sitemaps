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

use crate::limits::{BYTES_LIMIT, RECORDS_LIMIT};
use crate::Record;

pub trait BuilderStat {
    fn written_bytes(&self) -> usize;
    fn written_records(&self) -> usize;

    fn bytes_until_limit(&self) -> usize {
        BYTES_LIMIT - self.written_bytes()
    }

    fn records_until_limit(&self) -> usize {
        RECORDS_LIMIT - self.written_records()
    }

    fn is_ok(&self) -> bool {
        self.bytes_until_limit() > 0 && self.records_until_limit() > 0
    }
}

pub trait Builder<W: Write, D: Record>: Sized {
    type Error: Error;

    fn new(writer: W) -> Result<Self, Self::Error>;
    fn next(&mut self, record: &D) -> Result<(), Self::Error>;
    fn finalize(self) -> Result<W, Self::Error>;
}

pub trait IteratorBuilder<'re, W: Write, D: Record + 're>: Builder<W, D> {
    fn build(writer: W, records: impl Iterator<Item = &'re D>) -> Result<W, Self::Error>;
}

impl<'re, W: Write, D: Record + 're, T> IteratorBuilder<'re, W, D> for T
where
    T: Builder<W, D>,
{
    fn build(writer: W, records: impl Iterator<Item = &'re D>) -> Result<W, Self::Error> {
        let mut builder = Self::new(writer)?;
        for record in records {
            builder.next(record)?;
        }

        builder.finalize()
    }
}

pub trait StringBuilder<'re, D: Record + 're>: IteratorBuilder<'re, Vec<u8>, D> {
    fn build_string(records: impl Iterator<Item = &'re D>) -> Result<String, Self::Error>;
}

impl<'re, D: Record + 're, T> StringBuilder<'re, D> for T
where
    T: IteratorBuilder<'re, Vec<u8>, D>,
{
    fn build_string(records: impl Iterator<Item = &'re D>) -> Result<String, Self::Error> {
        // TODO avoid extra copy
        let buffer = Self::build(Vec::new(), records)?;
        let buffer = String::from_utf8_lossy(buffer.as_slice());
        Ok(buffer.to_string())
    }
}

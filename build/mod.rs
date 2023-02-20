use std::error::Error;
use std::io::Write;

use crate::{IndexRecord, SitemapRecord};

mod txt;
pub use txt::{TxtBuilder, TxtBuilderError};

#[cfg(feature = "xml")]
mod xml;
#[cfg(feature = "xml")]
pub use xml::{XmlBuilder, XmlBuilderError};

#[cfg(feature = "rss")]
mod rss;
#[cfg(feature = "rss")]
pub use rss::{RssBuilder, RssBuilderError};

pub trait SitemapBuilder<W: Write>: Sized {
    type Error: Error;

    fn build<'re>(
        writer: W,
        records: impl Iterator<Item = &'re SitemapRecord>,
    ) -> Result<W, Self::Error> {
        let mut state = Self::create(writer)?;
        for record in records {
            state.next(record)?;
        }

        state.finalize()
    }

    fn create(writer: W) -> Result<Self, Self::Error>;
    fn next(&mut self, record: &SitemapRecord) -> Result<(), Self::Error>;
    fn finalize(self) -> Result<W, Self::Error>;
}

pub trait SitemapBuilderString: SitemapBuilder<Vec<u8>> {
    fn build_string<'re>(
        records: impl Iterator<Item = &'re SitemapRecord>,
    ) -> Result<String, Self::Error>;
}

impl<T> SitemapBuilderString for T
where
    T: SitemapBuilder<Vec<u8>>,
{
    fn build_string<'re>(
        records: impl Iterator<Item = &'re SitemapRecord>,
    ) -> Result<String, Self::Error> {
        let buffer = Vec::new();
        let buffer = Self::build(buffer, records)?;
        Ok(String::from_utf8_lossy(buffer.as_slice()).to_string())
    }
}

pub trait IndexBuilder<W: Write>: Sized {
    type Error: Error;

    fn build_index<'re>(
        writer: W,
        records: impl Iterator<Item = &'re IndexRecord>,
    ) -> Result<W, Self::Error> {
        let mut state = Self::create(writer)?;
        for record in records {
            state.next(record)?;
        }

        state.finalize()
    }

    fn create(writer: W) -> Result<Self, Self::Error>;
    fn next(&mut self, record: &IndexRecord) -> Result<(), Self::Error>;
    fn finalize(self) -> Result<W, Self::Error>;
}

pub trait IndexBuilderString: IndexBuilder<Vec<u8>> {
    fn build_index_string<'re>(
        records: impl Iterator<Item = &'re IndexRecord>,
    ) -> Result<String, Self::Error>;
}

impl<T> IndexBuilderString for T
where
    T: IndexBuilder<Vec<u8>>,
{
    fn build_index_string<'re>(
        records: impl Iterator<Item = &'re IndexRecord>,
    ) -> Result<String, Self::Error> {
        let buffer = Vec::new();
        let buffer = Self::build_index(buffer, records)?;
        Ok(String::from_utf8_lossy(buffer.as_slice()).to_string())
    }
}

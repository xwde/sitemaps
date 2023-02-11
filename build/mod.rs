use crate::record::{IndexRecord, SitemapRecord};

use std::error::Error;
use std::io::Write;

mod txt;
pub use txt::*;

#[cfg(feature = "xml")]
mod xml;
#[cfg(feature = "xml")]
pub use xml::*;

pub trait SitemapState<W: Write>: Sized {
    type Error: Error;

    fn create(writer: W) -> Result<Self, Self::Error>;
    fn next(&mut self, record: &SitemapRecord) -> Result<(), Self::Error>;
    fn finalize(&mut self) -> Result<(), Self::Error>;
}

pub trait SitemapBuilder<W: Write> {
    type State: SitemapState<W>;

    fn build<'re>(
        writer: W,
        records: impl Iterator<Item = &'re SitemapRecord>,
    ) -> Result<(), <Self::State as SitemapState<W>>::Error> {
        let mut state = Self::State::create(writer)?;
        for record in records {
            state.next(record)?;
        }

        state.finalize()
    }
}

pub trait IndexState<W: Write>: Sized {
    type Error: Error;

    fn create(writer: W) -> Result<Self, Self::Error>;
    fn next(&mut self, record: &IndexRecord) -> Result<(), Self::Error>;
    fn finalize(&mut self) -> Result<(), Self::Error>;
}

pub trait IndexBuilder<W: Write> {
    type State: IndexState<W>;

    fn build<'re>(
        writer: W,
        records: impl Iterator<Item = &'re IndexRecord>,
    ) -> Result<(), <Self::State as IndexState<W>>::Error> {
        let mut state = Self::State::create(writer)?;
        for record in records {
            state.next(record)?;
        }

        state.finalize()
    }
}

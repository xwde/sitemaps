use std::error::Error;
use std::io::Read;

use crate::record::SitemapRecord;

mod txt;
pub use txt::*;

#[cfg(feature = "xml")]
mod xml;
#[cfg(feature = "xml")]
pub use xml::*;

#[cfg(feature = "rss")]
mod rss;
#[cfg(feature = "rss")]
pub use rss::*;

pub trait SitemapState<R: Read>: Sized {
    type Error: Error;

    fn create(reader: R) -> Result<Self, Self::Error>;
    fn next(&mut self) -> Result<Option<SitemapRecord>, Self::Error>;
}

pub trait SitemapParser<R: Read> {
    type State: SitemapState<R>;

    fn parse(reader: R) -> Result<(), <Self::State as SitemapState<R>>::Error> {
        // let mut state = Self::State::create(reader);
        // state.next(record)?;
        todo!()
    }
}

// pub trait IndexParser {}

// pub trait AutoParser {}

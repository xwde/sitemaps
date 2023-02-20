use std::error::Error;
use std::io::Read;

use crate::{IndexRecord, SitemapRecord};

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

pub trait SitemapParser<R: Read>: Sized {
    type Error: Error;

    fn parse(reader: R) -> Result<(), Self::Error> {
        todo!()
    }

    fn create(reader: R) -> Result<Self, Self::Error>;
    fn next(&mut self) -> Result<Option<SitemapRecord>, Self::Error>;
}

pub trait IndexParser<R: Read>: Sized {
    type Error: Error;

    fn parse(reader: R) -> Result<(), Self::Error> {
        todo!()
    }

    fn create(reader: R) -> Result<Self, Self::Error>;
    fn next(&mut self) -> Result<Option<IndexRecord>, Self::Error>;
}

// pub trait AutoParser<R: Read>: SitemapParser<R> + IndexParser<R> {}

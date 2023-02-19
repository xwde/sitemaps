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
    ) -> Result<(), Self::Error> {
        let mut state = Self::create(writer)?;
        for record in records {
            state.next(record)?;
        }

        state.finalize()
    }

    // fn build_string<'re>(
    //     records: impl Iterator<Item = &'re SitemapRecord>,
    // ) -> String {
    //     let mut buffer = Vec::new();
    //     Self::build(&mut buffer, records).u;
    //     String::from_utf8_lossy(buffer.as_slice()).to_string()
    // }

    fn create(writer: W) -> Result<Self, Self::Error>;
    fn next(&mut self, record: &SitemapRecord) -> Result<(), Self::Error>;
    fn finalize(&mut self) -> Result<(), Self::Error>;
}

// pub trait SitemapBuilder0<'bld>: SitemapBuilder<&'bld mut Vec<u8>> {
//     fn build_string<'re0>(
//         records: impl Iterator<Item = &'re0 SitemapRecord>,
//     ) -> Result<String, Self::Error> {
//         let mut buffer = Vec::new();
//         Self::build(&mut buffer, records)?;
//         Ok(String::from_utf8_lossy(buffer.as_slice()).to_string())
//     }
// }

// pub trait SitemapBuilder0 {
//     type Builder<'bld>: SitemapBuilder<&'bld mut Vec<u8>>;
//
//     fn build_string<'re0, 'bld>(
//         records: impl Iterator<Item = &'re0 SitemapRecord>,
//     ) -> Result<String, <Self::Builder<'bld> as SitemapBuilder<&'bld mut Vec<u8>>>::Error> {
//         let mut buffer = Vec::new();
//         Self::Builder::build(&mut buffer, records)?;
//         Ok(String::from_utf8_lossy(buffer.as_slice()).to_string())
//     }
// }

pub trait IndexBuilder<W: Write>: Sized {
    type Error: Error;

    fn build_index<'re>(
        writer: W,
        records: impl Iterator<Item = &'re IndexRecord>,
    ) -> Result<(), Self::Error> {
        let mut state = Self::create(writer)?;
        for record in records {
            state.next(record)?;
        }

        state.finalize()
    }

    fn create(writer: W) -> Result<Self, Self::Error>;
    fn next(&mut self, record: &IndexRecord) -> Result<(), Self::Error>;
    fn finalize(&mut self) -> Result<(), Self::Error>;
}

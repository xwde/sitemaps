use crate::entry::{SitemapEntry, SitemapIndexEntry};
use std::error::Error;

mod rss;
mod txt;
mod xml;

pub use rss::*;
pub use txt::*;
pub use xml::*;

// pub trait FormatEntry {
//     fn deserialize(&self, sitemap: &str) -> impl Iterator<Item = SitemapEntry>;
//     fn serialize(&self, container: impl Iterator<Item = SitemapEntry>) -> String;
// }

// pub trait FormatIndexEntry {
//     type ReadError: Error;
//     type WriteError: Error;
//
//     fn deserialize(&self, sitemap: &str) -> Result<Self, Self::ReadError>;
//     fn serialize(
//         &self,
//         container: impl Iterator<Item = SitemapIndexEntry>,
//     ) -> Result<String, Self::WriteError>;
// }

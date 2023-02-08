use crate::record::{SitemapEntry, SitemapIndexEntry};

mod txt;
pub use txt::*;

#[cfg(feature = "rss")]
mod rss;
#[cfg(feature = "rss")]
pub use rss::*;

#[cfg(feature = "xml")]
mod xml;
#[cfg(feature = "xml")]
pub use xml::*;

pub trait ConvertSitemap {
    fn deserialize_sitemap<'de>(
        &self,
        sitemap: &'de str,
    ) -> impl Iterator<Item = SitemapEntry> + 'de;
    fn serialize_sitemap(&self, container: impl Iterator<Item = SitemapEntry>) -> String;
}

pub trait ConvertIndex {
    fn deserialize_idx<'de>(&self, idx: &'de str) -> impl Iterator<Item = SitemapIndexEntry> + 'de;
    fn serialize_idx(&self, container: impl Iterator<Item = SitemapIndexEntry>) -> String;
}

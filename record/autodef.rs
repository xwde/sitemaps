use crate::{IndexRecord, Record, SitemapRecord};

#[derive(Debug, Clone, PartialEq)]
pub enum AutodefRecord {
    Sitemap(SitemapRecord),
    Index(IndexRecord),
}

impl AutodefRecord {
    pub fn from_sitemap(record: SitemapRecord) -> Self {
        Self::Sitemap(record)
    }

    pub fn from_index(record: IndexRecord) -> Self {
        Self::Index(record)
    }
}

impl Record for AutodefRecord {}

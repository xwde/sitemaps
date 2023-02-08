use crate::record::{SitemapEntry, SitemapIndexEntry};
use crate::{ConvertIndex, ConvertSitemap};

#[derive(Debug, Clone, Default)]
pub struct XmlConverter {}

impl XmlConverter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ConvertSitemap for XmlConverter {
    fn deserialize_sitemap<'de>(
        &self,
        sitemap: &'de str,
    ) -> impl Iterator<Item = SitemapEntry> + 'de {
        todo!()
    }

    fn serialize_sitemap(&self, container: impl Iterator<Item = SitemapEntry>) -> String {
        todo!()
    }
}

impl ConvertIndex for XmlConverter {
    fn deserialize_idx<'de>(
        &self,
        index: &'de str,
    ) -> impl Iterator<Item = SitemapIndexEntry> + 'de {
        todo!()
    }

    fn serialize_idx(&self, container: impl Iterator<Item = SitemapIndexEntry>) -> String {
        todo!()
    }
}

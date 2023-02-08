use crate::record::SitemapEntry;
use crate::ConvertSitemap;

#[derive(Debug, Clone, Default)]
pub struct RssConverter {}

impl RssConverter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ConvertSitemap for RssConverter {
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

use crate::attribute::AsUnderlying;
use crate::record::SitemapEntry;
use crate::ConvertSitemap;

#[derive(Debug, Clone, Default)]
pub struct TxtConverter {}

impl TxtConverter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ConvertSitemap for TxtConverter {
    fn deserialize_sitemap<'de>(
        &self,
        sitemap: &'de str,
    ) -> impl Iterator<Item = SitemapEntry> + 'de {
        sitemap
            .lines()
            .map(SitemapEntry::parse)
            .filter_map(|e| e.ok())
    }

    fn serialize_sitemap(&self, container: impl Iterator<Item = SitemapEntry>) -> String {
        container
            .map(|e| e.location.as_underlying().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

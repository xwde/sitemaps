use crate::attribute::AsUnderlying;
use crate::entry::SitemapEntry;

pub struct TxtFormat {}

impl TxtFormat {
    pub fn new() -> Self {
        todo!()
    }

    fn deserialize<'a>(&'a self, sitemap: &'a str) -> impl Iterator<Item = SitemapEntry> + 'a {
        sitemap
            .lines()
            .map(|e| SitemapEntry::parse(e))
            .filter_map(|e| e.ok())
    }

    fn serialize(&self, container: impl Iterator<Item = SitemapEntry>) -> String {
        container
            .map(|e| e.location.as_underlying().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

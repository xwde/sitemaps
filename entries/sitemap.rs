use crate::entries::sitemap_entry::SitemapEntry;

#[cfg(feature = "serde")]
use quick_xml::{de::from_str, se::to_string, DeError};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Sitemap {
    #[cfg_attr(feature = "serde", serde(rename = "url"))]
    pub entries: Vec<SitemapEntry>,
}

#[cfg(feature = "serde")]
impl Sitemap {
    pub fn deserialize(xml: &str) -> Result<Self, DeError> {
        from_str(xml)
    }

    pub fn serialize(&self) -> Result<String, DeError> {
        to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::entries::sitemap::Sitemap;

    #[test]
    fn deserialize() {
        let xml = r#"
        <urlset xmlns="https://www.sitemaps.org/schemas/sitemap/0.9">
            <url>
                <loc>https://example.com/</loc>
                <lastmod>2023-01-10T22:11:17.000-05:00</lastmod>
                <changefreq>daily</changefreq>
                <priority>0.8</priority>
            </url>
        </urlset>"#;
        let rs = Sitemap::deserialize(xml).unwrap();
        dbg!(rs);
    }
}

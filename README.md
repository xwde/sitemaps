### xwde: sitemap

> **Warning** : The library is in active development. Expect breaking changes.

The implementation of the Sitemap protocol (or URL inclusion protocol) in the
Rust programming language with the support of `txt`, `xml` formats and `video`,
`image`, `news` extensions (according to the Google's spec).

Following features available:

- `txt` to enable .txt parser & builder.
- `xml` to enable .xml parser & builder.
- `extension` to enable all extensions.

#### parsing

```rust
fn main() {
  
}
```

#### building

```rust
use sitemaps::{Record, SitemapRecord};
use sitemaps::{Builder, XmlBuilder};
use sitemaps::attribute::{Attribute, Location};

fn main() {
    let mut buffer = Vec::new();
   
    // Replace XmlBuilder with TxtBuilder for Txt Sitemap.
    let mut builder = XmlBuilder::initialize(&mut buffer).unwrap();

    // Replace SitemapRecord with IndexRecord for Sitemap Index.
    let record = "https://example.com/";
    let record = Location::parse(record).unwrap();
    let record = SitemapRecord::new(record);

    builder.next(record).unwrap();
    builder.finalize().unwrap();

    let sitemap = String::from_utf8_lossy(buffer.as_slice());
    println!(sitemap.to_string());
}
```

#### read more

- [Sitemaps Overview](https://developers.google.com/search/docs/crawling-indexing/sitemaps/overview)
  on Google.com
- [Sitemaps Best Practice](https://developers.google.com/search/blog/2014/10/best-practices-for-xml-sitemaps-rssatom)
  on Google.com
- [Deprecated Attributes](https://developers.google.com/search/blog/2022/05/spring-cleaning-sitemap-extensions)
  on Google.com

- [Sitemaps](https://en.wikipedia.org/wiki/Sitemaps) on Wikipedia.org
- [Sitemaps Format](https://www.sitemaps.org/protocol.html) on Sitemap.org
- [Sitemaps FAQ](https://www.sitemaps.org/faq.htm) on Sitemap.org

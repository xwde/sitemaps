### Sitemaps

The implementation of the Sitemap protocol (or URL inclusion protocol) in the
Rust programming language with the support of `txt`, `xml`, `rss` formats and
`video`, `image`, and `news` extensions (according to Google's specifications).

#### Parser

```rust
fn main() {
  
}
```

#### Builder

```rust
use sitemaps::build::{SitemapBuilderString, XmlBuilder};
use sitemaps::SitemapRecord;

fn main() {
    let uri = "https://www.example.com/";
    let record = SitemapRecord::parse(uri).unwrap();
    let records = vec![record /* & more records... */];
    let sitemap = XmlBuilder::build_string(records.iter()).unwrap();
    println!("{}", sitemap);
}
```

#### Links

- [Sitemaps Overview](https://developers.google.com/search/docs/crawling-indexing/sitemaps/overview)
  on Google.com
- [Sitemaps](https://en.wikipedia.org/wiki/Sitemaps) on Wikipedia.org
- [Sitemaps Format](https://www.sitemaps.org/protocol.html) on Sitemap.org
- [Sitemaps FAQ](https://www.sitemaps.org/faq.htm) on Sitemap.org

use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Write;

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Error as XmlError, Writer};

use crate::attribute::AsUnderlying;
use crate::build::{IndexBuilder, SitemapBuilder};
use crate::{IndexRecord, SitemapRecord};

#[derive(Debug)]
pub enum XmlBuilderError {
    TooManyRecords(usize),
    TooManyBytes(usize),
    XmlError(XmlError),
}

impl Display for XmlBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}

impl From<XmlError> for XmlBuilderError {
    fn from(error: XmlError) -> Self {
        XmlBuilderError::XmlError(error)
    }
}

impl Error for XmlBuilderError {}

pub struct XmlBuilder<W: Write> {
    writer: Writer<W>,
}

impl<W: Write> XmlBuilder<W> {
    const XMLNS: [(&'static str, &'static str); 1] =
        [("xmlns", "http://www.sitemaps.org/schemas/sitemap/0.9")];

    const URL_SET: &'static str = "urlset";
    const URL: &'static str = "url";

    const SITEMAP_INDEX: &'static str = "sitemapindex";
    const SITEMAP: &'static str = "sitemap";

    const LOCATION: &'static str = "loc";
    const LAST_MODIFIED: &'static str = "lastmod";
    const CHANGE_FREQUENCY: &'static str = "changefreq";
    const PRIORITY: &'static str = "priority";
}

impl<W: Write> SitemapBuilder<W> for XmlBuilder<W> {
    type Error = XmlBuilderError;

    fn create(writer: W) -> Result<Self, Self::Error> {
        let mut writer = Writer::new(writer);
        writer.write_bom()?;

        let tag = BytesStart::new(Self::URL_SET);
        let tag = tag.with_attributes(Self::XMLNS);
        writer.write_event(Event::Start(tag))?;

        Ok(Self { writer })
    }

    fn next(&mut self, record: &SitemapRecord) -> Result<(), Self::Error> {
        let tag = self.writer.create_element(Self::URL);
        tag.write_inner_content(|writer| {
            {
                let location = record.location.to_string();
                let location = location.as_str();
                let tag = writer.create_element(Self::LOCATION);
                tag.write_text_content(BytesText::new(location))?;
            }

            if let Some(last_modified) = &record.last_modified {
                let last_modified = last_modified.to_string();
                let last_modified = last_modified.as_str();
                let tag = writer.create_element(Self::LAST_MODIFIED);
                tag.write_text_content(BytesText::new(last_modified))?;
            }

            if let Some(change_frequency) = &record.change_frequency {
                let change_frequency = change_frequency.as_underlying();
                let tag = writer.create_element(Self::CHANGE_FREQUENCY);
                tag.write_text_content(BytesText::new(change_frequency))?;
            }

            if let Some(priority) = &record.priority {
                let priority = priority.to_string();
                let priority = priority.as_str();
                let tag = writer.create_element(Self::PRIORITY);
                tag.write_text_content(BytesText::new(priority))?;
            }

            // TODO other tags

            Ok(())
        })?;

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), Self::Error> {
        let tag = BytesEnd::new(Self::URL_SET);
        self.writer.write_event(Event::End(tag))?;

        Ok(())
    }
}

impl<W: Write> IndexBuilder<W> for XmlBuilder<W> {
    type Error = XmlBuilderError;

    fn create(writer: W) -> Result<Self, Self::Error> {
        let mut writer = Writer::new(writer);
        writer.write_bom()?;

        let tag = BytesStart::new(Self::SITEMAP_INDEX);
        let tag = tag.with_attributes(Self::XMLNS);
        writer.write_event(Event::Start(tag))?;

        Ok(Self { writer })
    }

    fn next(&mut self, record: &IndexRecord) -> Result<(), Self::Error> {
        let tag = self.writer.create_element(Self::SITEMAP);
        tag.write_inner_content(|writer| {
            {
                let location = record.location.to_string();
                let location = location.as_str();
                let tag = writer.create_element(Self::LOCATION);
                tag.write_text_content(BytesText::new(location))?;
            }

            if let Some(last_modified) = &record.last_modified {
                let last_modified = last_modified.to_string();
                let last_modified = last_modified.as_str();
                let tag = writer.create_element(Self::LAST_MODIFIED);
                tag.write_text_content(BytesText::new(last_modified))?;
            }

            Ok(())
        })?;

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), Self::Error> {
        let tag = BytesEnd::new(Self::SITEMAP_INDEX);
        self.writer.write_event(Event::End(tag))?;

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     fn create(records: Vec<SitemapRecord>) -> String {
//         let mut buffer = vec![];
//         XmlBuilder::build(&mut buffer, records.iter()).unwrap();
//         String::from_utf8_lossy(buffer.as_slice()).to_string()
//     }
//
//     #[test]
//     fn create_1() {
//         let record = SitemapRecord::parse("https://www.example.com/").unwrap();
//         let records = vec![record.clone(), record.clone(), record.clone()];
//         let sitemap = create(records);
//         println!("{}", sitemap);
//     }
// }

use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Read;

use quick_xml::Reader;

use crate::parse::SitemapParser;
use crate::SitemapRecord;

#[derive(Debug)]
pub struct XmlParserError {}

impl Display for XmlParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}

impl Error for XmlParserError {}

pub struct XmlParser<R: Read> {
    reader: Reader<R>,
}

impl<R: Read> SitemapParser<R> for XmlParser<R> {
    type Error = XmlParserError;

    fn create(reader: R) -> Result<Self, Self::Error> {
        let reader = Reader::from_reader(reader);
        Ok(Self { reader })
    }

    fn next(&mut self) -> Result<Option<SitemapRecord>, Self::Error> {
        todo!()
    }
}

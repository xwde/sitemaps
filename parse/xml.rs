use quick_xml::Reader;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Read;

use crate::parse::{SitemapParser, SitemapState};
use crate::SitemapRecord;

#[derive(Debug)]
pub struct XmlParserError {}

impl Display for XmlParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}

impl Error for XmlParserError {}

pub struct XmlState<R: Read> {
    reader: Reader<R>,
}

impl<R: Read> SitemapState<R> for XmlState<R> {
    type Error = XmlParserError;

    fn create(reader: R) -> Result<Self, Self::Error> {
        todo!()
    }

    fn next(&mut self) -> Result<Option<SitemapRecord>, Self::Error> {
        todo!()
    }
}

pub struct XmlParser {}

impl<R: Read> SitemapParser<R> for XmlParser {
    type State = XmlState<R>;
}

use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Read;

use quick_xml::Reader;

use crate::parse::SitemapParser;
use crate::SitemapRecord;

#[derive(Debug)]
pub struct RssParserError {}

impl Display for RssParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}

impl Error for RssParserError {}

pub struct RssParser<R: Read> {
    reader: Reader<R>,
}

impl<R: Read> SitemapParser<R> for RssParser<R> {
    type Error = RssParserError;

    fn create(reader: R) -> Result<Self, Self::Error> {
        let reader = Reader::from_reader(reader);
        Ok(Self { reader })
    }

    fn next(&mut self) -> Result<Option<SitemapRecord>, Self::Error> {
        todo!()
    }
}

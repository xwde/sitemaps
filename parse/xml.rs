use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Read;
use std::marker::PhantomData;

use quick_xml::Reader;

use crate::parse::Parser;
use crate::{IndexRecord, Record, SitemapRecord};

#[derive(Debug)]
pub enum XmlParserError {}

impl Display for XmlParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}

impl Error for XmlParserError {}

///
///
/// ```rust
/// ```
pub struct XmlParser<R: Read, D: Record> {
    record: PhantomData<D>,
    read_bytes: usize,
    read_records: usize,
    reader: Reader<R>,
}

impl<R: Read, D: Record> XmlParser<R, D> {}

impl<R: Read> Parser<R, SitemapRecord> for XmlParser<R, SitemapRecord> {
    type Error = XmlParserError;

    fn initialize(reader: R) -> Result<Self, Self::Error> {
        todo!()
    }

    fn next(&mut self) -> Result<Option<SitemapRecord>, Self::Error> {
        todo!()
    }

    fn finalize(self) -> Result<R, Self::Error> {
        todo!()
    }

    fn read_bytes(&self) -> usize {
        todo!()
    }

    fn read_records(&self) -> usize {
        todo!()
    }
}

impl<R: Read> Parser<R, IndexRecord> for XmlParser<R, IndexRecord> {
    type Error = XmlParserError;

    fn initialize(reader: R) -> Result<Self, Self::Error> {
        todo!()
    }

    fn next(&mut self) -> Result<Option<IndexRecord>, Self::Error> {
        todo!()
    }

    fn finalize(self) -> Result<R, Self::Error> {
        todo!()
    }

    fn read_bytes(&self) -> usize {
        todo!()
    }

    fn read_records(&self) -> usize {
        todo!()
    }
}

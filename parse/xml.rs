use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Read;
use std::marker::PhantomData;

use quick_xml::Reader;

use crate::{Record, SitemapRecord};

#[derive(Debug)]
pub struct XmlParserError {}

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
    reader: Reader<R>,
}

impl<R: Read, D: Record> XmlParser<R, D> {}

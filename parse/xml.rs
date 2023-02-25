use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Read;
use std::marker::PhantomData;

use quick_xml::Reader;

use crate::parse::{Parser, ParserStat};
use crate::{AutodefRecord, IndexRecord, Record, SitemapRecord};

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
    read_bytes: usize,
    read_records: usize,
    reader: Reader<R>,
}

impl<R: Read, D: Record> XmlParser<R, D> {
    fn new(reader: R) -> Self {
        let reader = Reader::from_reader(reader);
        Self {
            record: PhantomData,
            read_bytes: 0,
            read_records: 0,
            reader,
        }
    }

    fn try_next(&mut self) -> Result<Option<AutodefRecord>, XmlParserError> {
        todo!()
    }
}

impl<R: Read, D: Record> ParserStat for XmlParser<R, D> {
    fn read_bytes(&self) -> usize {
        self.read_bytes
    }

    fn read_records(&self) -> usize {
        self.read_records
    }
}

impl<R: Read> Parser<R, SitemapRecord> for XmlParser<R, SitemapRecord> {
    type Error = XmlParserError;

    fn new(reader: R) -> Result<Self, Self::Error> {
        todo!()
    }

    fn next(&mut self) -> Result<Option<SitemapRecord>, Self::Error> {
        todo!()
    }

    fn finalize(self) -> Result<R, Self::Error> {
        todo!()
    }
}

impl<R: Read> Parser<R, IndexRecord> for XmlParser<R, IndexRecord> {
    type Error = XmlParserError;

    fn new(reader: R) -> Result<Self, Self::Error> {
        todo!()
    }

    fn next(&mut self) -> Result<Option<IndexRecord>, Self::Error> {
        todo!()
    }

    fn finalize(self) -> Result<R, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::{Parser, XmlParser};
    use crate::SitemapRecord;

    #[test]
    fn foo() {
        // Pretend it's our reader.
        let mut buffer = "https://example.com/".as_bytes();

        // Replace XmlParser with TxtParser for Xml Sitemap.
        let mut parser = XmlParser::<_, SitemapRecord>::new(&mut buffer).unwrap();
        // let record: SitemapRecord = parser.next().unwrap().unwrap();

        while let Some(record) = parser.next().ok().flatten() {
            println!("{}", record.location.to_string());
        }

        parser.finalize().unwrap();
    }
}

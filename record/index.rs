use crate::attribute::LastModified;
use crate::attribute::Location;
use crate::Record;

#[derive(Debug, Clone, PartialEq)]
pub struct IndexRecord {
    pub location: Location,
    pub last_modified: Option<LastModified>,
}

impl Record for IndexRecord {
    fn new(location: Location) -> Self {
        Self {
            location,
            last_modified: None,
        }
    }
}

impl IndexRecord {
    pub fn with_timestamp(&mut self, modified: LastModified) -> &mut Self {
        self.last_modified = Some(modified);
        self
    }
}

impl From<Location> for IndexRecord {
    fn from(location: Location) -> Self {
        Self::new(location)
    }
}

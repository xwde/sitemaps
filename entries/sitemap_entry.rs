use crate::attributes::frequency::ChangeFrequency;
// use crate::attributes::location::Location;
// use crate::attributes::modification::LastModified;
use crate::attributes::priority::Priority;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SitemapEntry {
    #[cfg_attr(feature = "serde", serde(rename = "loc"))]
    pub location: String,
    // pub location: Location,
    #[cfg_attr(feature = "serde", serde(rename = "changefreq"))]
    pub change_frequency: Option<ChangeFrequency>,
    #[cfg_attr(feature = "serde", serde(rename = "lastmod"))]
    pub last_modified: Option<String>,
    // pub last_modified: Option<LastModified>,
    #[cfg_attr(feature = "serde", serde(rename = "priority"))]
    pub priority: Option<Priority>,
}

impl SitemapEntry {
    // pub fn new(location: Location) -> Self {
    //     Self {
    //         location,
    //         change_frequency: None,
    //         last_modified: None,
    //         priority: None,
    //     }
    // }

    // pub fn with_change_frequency(&mut self, frequency: ChangeFrequency) -> &mut Self {
    //     self.change_frequency = Some(frequency);
    //     self
    // }

    // pub fn with_last_modified(&mut self, last_modified: LastModified) -> &mut Self {
    //     self.last_modified = Some(last_modified);
    //     self
    // }

    pub fn with_priority(&mut self, priority: Priority) -> &mut Self {
        self.priority = Some(priority);
        self
    }
}

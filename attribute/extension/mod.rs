mod image;
mod news;
mod video;

pub use self::image::*;
pub use self::news::*;
pub use self::video::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Extension {
    image: Option<Image>,
    video: Option<Video>,
    news: Option<News>,
}

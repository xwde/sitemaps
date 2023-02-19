use time::OffsetDateTime;

#[derive(Debug, PartialEq, Clone)]
pub struct News {
    pub name: String,
    pub language: String,
    pub date: OffsetDateTime,
    pub title: String,
}

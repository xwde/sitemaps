use isolang::Language;
use time::OffsetDateTime;

// pub struct Language {}

#[derive(Debug, Clone, PartialEq)]
pub struct News {
    title: String,
    name: String,
    language: Language,
    published: OffsetDateTime,
}

impl News {
    fn parse(title: &str, name: &str, language: &str, date: &str) -> Self {
        todo!()
    }
}

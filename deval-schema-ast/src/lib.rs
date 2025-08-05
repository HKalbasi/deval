use std::ops::Range;

#[derive(Debug)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub struct RecordMatcher {
    pub key: String,
    pub docs: String,
    pub value: DataMatcher,
}

#[derive(Debug)]
pub enum DataMatcher {
    Ident(Spanned<String>),
    Array { element: Box<DataMatcher> },
    Object(Vec<RecordMatcher>),
}

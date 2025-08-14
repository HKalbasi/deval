use std::ops::Range;

#[derive(Debug)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub enum RecordMatcher {
    SimpleKey {
        key: String,
        docs: String,
        value: DataMatcher,
    },
    AnyKey,
}

#[derive(Debug)]
pub enum DataMatcher {
    Ident(Spanned<String>),
    Array { element: Box<DataMatcher> },
    Object(Vec<RecordMatcher>),
    Union(Vec<DataMatcher>),
}

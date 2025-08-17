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
        optional: bool,
        docs: String,
        value: Expression,
    },
    AnyKey,
}

#[derive(Debug)]
pub enum Expression {
    Number(Spanned<f64>),
    Range {
        start: Option<Spanned<Box<Expression>>>,
        end: Option<Spanned<Box<Expression>>>,
        is_inclusive: bool,
    },
    Ident(Spanned<String>),
    Array {
        element: Box<Expression>,
        index: Option<Spanned<Box<Expression>>>,
    },
    Object(Vec<RecordMatcher>),
    Union(Vec<Expression>),
}

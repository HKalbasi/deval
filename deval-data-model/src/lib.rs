
#[derive(Debug)]
pub struct Span {
    pub filename: String,
    /// start offset in bytes
    pub start: usize,
    /// end offset in bytes
    pub end: usize,
}

#[derive(Debug)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

#[derive(Debug)]
pub enum SpannedData {
    Null,
    Bool(Spanned<bool>),
    Number(Spanned<f64>),
    String(Spanned<String>),
    Array(Vec<Spanned<SpannedData>>),
    Object(Vec<(Spanned<String>, Spanned<SpannedData>)>),
}

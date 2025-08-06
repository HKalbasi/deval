#[derive(Debug, Clone)]
pub struct Span {
    pub filename: String,
    /// start offset in bytes
    pub start: usize,
    /// end offset in bytes
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum SpannedData {
    Null,
    Bool(Spanned<bool>),
    Number(Spanned<f64>),
    String(Spanned<String>),
    Array(Vec<Spanned<SpannedData>>),
    Object(Vec<(Spanned<String>, Spanned<SpannedData>)>),
}

impl SpannedData {
    pub fn kind(&self) -> &'static str {
        match self {
            SpannedData::Null => "Null",
            SpannedData::Bool(_) => "Bool",
            SpannedData::Number(_) => "Number",
            SpannedData::String(_) => "String",
            SpannedData::Array(_) => "Array",
            SpannedData::Object(_) => "Object",
        }
    }
}

#[derive(Debug)]
pub struct Annotated<T> {
    pub value: T,
    pub span: Span,
    pub docs: String,
}

impl<A, B: From<A>> From<Spanned<A>> for Annotated<B> {
    fn from(spanned: Spanned<A>) -> Self {
        Annotated {
            value: spanned.value.into(),
            span: spanned.span,
            docs: String::new(),
        }
    }
}

#[derive(Debug)]
pub enum AnnotatedData {
    Null,
    Bool(Annotated<bool>),
    Number(Annotated<f64>),
    String(Annotated<String>),
    Array(Vec<Annotated<AnnotatedData>>),
    Object(Vec<(Annotated<String>, Annotated<AnnotatedData>)>),
}

impl From<SpannedData> for AnnotatedData {
    fn from(value: SpannedData) -> Self {
        match value {
            SpannedData::Null => AnnotatedData::Null,
            SpannedData::Bool(spanned) => AnnotatedData::Bool(spanned.into()),
            SpannedData::Number(spanned) => AnnotatedData::Number(spanned.into()),
            SpannedData::String(spanned) => AnnotatedData::String(spanned.into()),
            SpannedData::Array(spanneds) => {
                AnnotatedData::Array(spanneds.into_iter().map(|x| x.into()).collect())
            }
            SpannedData::Object(items) => AnnotatedData::Object(
                items
                    .into_iter()
                    .map(|(key, value)| (key.into(), value.into()))
                    .collect(),
            ),
        }
    }
}

pub struct ParseError {
    pub message: String,
    pub span: Span,
}

pub trait Format {
    fn parse(&self, source: &str, filename: &str) -> Result<Spanned<SpannedData>, Vec<ParseError>>;
}
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Span {
    pub filename: String,
    /// start offset in bytes
    pub start: usize,
    /// end offset in bytes
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct SpanSet(pub Vec<Span>);

impl SpanSet {
    pub fn primary(&self) -> Span {
        self.0[0].clone()
    }
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub value: T,
    pub span: SpanSet,
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

#[derive(Debug, Clone, Copy)]
pub enum SemanticType {
    String,
    Number,
    Variable,
}

#[derive(Debug)]
pub struct Annotated<T> {
    pub value: T,
    pub span: SpanSet,
    pub docs: String,
    pub semantic_type: Option<SemanticType>,
}

impl<A, B: From<A>> From<Spanned<A>> for Annotated<B> {
    fn from(spanned: Spanned<A>) -> Self {
        Annotated {
            value: spanned.value.into(),
            span: spanned.span,
            docs: String::new(),
            semantic_type: None,
        }
    }
}

impl<T> Annotated<T> {
    fn with_semnatic_type(self, semnatic: SemanticType) -> Self {
        Self {
            semantic_type: Some(semnatic),
            ..self
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

impl AnnotatedData {
    pub fn walk(&self, f: &mut impl FnMut(SpanSet, &str, Option<SemanticType>)) {
        fn for_annotated<T>(t: &Annotated<T>, f: &mut impl FnMut(SpanSet, &str, Option<SemanticType>)) {
            f(t.span.clone(), &t.docs, t.semantic_type);
        }
        match self {
            AnnotatedData::Null => (),
            AnnotatedData::Bool(annotated) => for_annotated(annotated, f),
            AnnotatedData::Number(annotated) => for_annotated(annotated, f),
            AnnotatedData::String(annotated) => for_annotated(annotated, f),
            AnnotatedData::Array(items) => {
                for item in items {
                    for_annotated(item, f);
                    item.value.walk(f);
                }
            },
            AnnotatedData::Object(items) => {
                for (key, value) in items {
                    for_annotated(key, f);
                    for_annotated(value, f);
                    value.value.walk(f);
                }
            },
        }
    }
}

impl From<SpannedData> for AnnotatedData {
    fn from(value: SpannedData) -> Self {
        match value {
            SpannedData::Null => AnnotatedData::Null,
            SpannedData::Bool(spanned) => AnnotatedData::Bool(Annotated::from(spanned)),
            SpannedData::Number(spanned) => AnnotatedData::Number(Annotated::from(spanned).with_semnatic_type(SemanticType::Number)),
            SpannedData::String(spanned) => AnnotatedData::String(Annotated::from(spanned).with_semnatic_type(SemanticType::String)),
            SpannedData::Array(spanneds) => {
                AnnotatedData::Array(spanneds.into_iter().map(|x| x.into()).collect())
            }
            SpannedData::Object(items) => AnnotatedData::Object(
                items
                    .into_iter()
                    .map(|(key, value)| {
                        (
                            Annotated::from(key).with_semnatic_type(SemanticType::Variable),
                            value.into(),
                        )
                    })
                    .collect(),
            ),
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

pub trait Format: Sync + Send {
    fn parse(&self, source: &str, filename: &str) -> Result<Spanned<SpannedData>, Vec<ParseError>>;
}
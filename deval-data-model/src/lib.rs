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

pub type Spanned<T> = Annotated<T, SpanSet>;
pub type SpannedData = AnnotatedData<SpanSet>;

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

#[derive(Debug, Clone)]
pub struct Annotated<T, A = FullAnnotation> {
    pub value: T,
    pub annotation: A,
}

#[derive(Debug, Clone)]
pub struct FullAnnotation {
    pub span: SpanSet,
    pub docs: String,
    pub semantic_type: Option<SemanticType>,
}

impl<A, B: From<A>> From<Spanned<A>> for Annotated<B, FullAnnotation> {
    fn from(spanned: Spanned<A>) -> Self {
        Annotated {
            value: spanned.value.into(),
            annotation: FullAnnotation {
                span: spanned.annotation,
                docs: String::new(),
                semantic_type: None,
            },
        }
    }
}

impl<T> Annotated<T, FullAnnotation> {
    fn with_semnatic_type(mut self, semnatic: SemanticType) -> Self {
        self.annotation.semantic_type = Some(semnatic);
        self
    }
}

impl<A> Annotated<AnnotatedData<A>, A> {
    pub fn discard_annotation(&self) -> Annotated<AnnotatedData<()>, ()> {
        Annotated {
            value: self.value.discard_annotation(),
            annotation: (),
        }
    }
}

impl<T: Clone, A> Annotated<T, A> {
    pub fn discard_annotation_shallow(&self) -> Annotated<T, ()> {
        Annotated {
            value: self.value.clone(),
            annotation: (),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AnnotatedData<A = FullAnnotation> {
    Null,
    Bool(Annotated<bool, A>),
    Number(Annotated<f64, A>),
    String(Annotated<String, A>),
    Array(Vec<Annotated<AnnotatedData<A>, A>>),
    Object(Vec<(Annotated<String, A>, Annotated<AnnotatedData<A>, A>)>),
}

impl<A> AnnotatedData<A> {
    pub fn walk(&self, f: &mut impl FnMut(A))
    where
        A: Clone,
    {
        fn for_annotated<T, A: Clone>(t: &Annotated<T, A>, f: &mut impl FnMut(A)) {
            f(t.annotation.clone());
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
            }
            AnnotatedData::Object(items) => {
                for (key, value) in items {
                    for_annotated(key, f);
                    for_annotated(value, f);
                    value.value.walk(f);
                }
            }
        }
    }

    fn discard_annotation(&self) -> AnnotatedData<()> {
        match self {
            AnnotatedData::Null => AnnotatedData::Null,
            AnnotatedData::Bool(annotated) => {
                AnnotatedData::Bool(annotated.discard_annotation_shallow())
            }
            AnnotatedData::Number(annotated) => {
                AnnotatedData::Number(annotated.discard_annotation_shallow())
            }
            AnnotatedData::String(annotated) => {
                AnnotatedData::String(annotated.discard_annotation_shallow())
            }
            AnnotatedData::Array(annotateds) => {
                AnnotatedData::Array(annotateds.iter().map(|x| x.discard_annotation()).collect())
            }
            AnnotatedData::Object(items) => AnnotatedData::Object(
                items
                    .iter()
                    .map(|x| (x.0.discard_annotation_shallow(), x.1.discard_annotation()))
                    .collect(),
            ),
        }
    }
}

impl From<SpannedData> for AnnotatedData<FullAnnotation> {
    fn from(value: SpannedData) -> Self {
        match value {
            SpannedData::Null => AnnotatedData::Null,
            SpannedData::Bool(spanned) => AnnotatedData::Bool(Annotated::from(spanned)),
            SpannedData::Number(spanned) => AnnotatedData::Number(
                Annotated::from(spanned).with_semnatic_type(SemanticType::Number),
            ),
            SpannedData::String(spanned) => AnnotatedData::String(
                Annotated::from(spanned).with_semnatic_type(SemanticType::String),
            ),
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

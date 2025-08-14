use chumsky::prelude::*;
use chumsky::text;

use deval_schema_ast::Spanned;
use deval_schema_ast::{DataMatcher, RecordMatcher};

pub type Error<'a> = chumsky::error::Rich<'a, char, SimpleSpan>;
pub use chumsky::span::SimpleSpan;

fn spanned<'a, T>(
    p: impl Parser<'a, &'a str, T, extra::Err<Error<'a>>> + Clone,
) -> impl Parser<'a, &'a str, Spanned<T>, extra::Err<Error<'a>>> + Clone {
    p.map_with(|t, ext| Spanned {
        value: t,
        span: ext.span().start..ext.span().end,
    })
}

fn parser<'a>() -> impl Parser<'a, &'a str, DataMatcher, extra::Err<Error<'a>>> {
    recursive(|data| {
        // Parse doc comments (/// lines)
        let doc_comment = just("///")
            .ignore_then(none_of("\n").repeated().collect::<String>())
            .padded();

        // Parse a record field: docs + key + colon + data type
        let record = doc_comment
            .repeated()
            .collect::<Vec<_>>()
            .map(|docs| docs.join("\n"))
            .then(text::ident().map(String::from))
            .then_ignore(just(':').padded())
            .then(data.clone())
            .map(|((docs, key), value)| RecordMatcher { key, docs, value });

        // Parse objects: { ... }
        let object = just('{')
            .padded()
            .ignore_then(
                record
                    .separated_by(just(',').padded())
                    .allow_trailing()
                    .collect::<Vec<_>>(),
            )
            .padded()
            .then_ignore(just('}').padded())
            .map(DataMatcher::Object);

        // Parse basic identifiers (string, number, etc.)
        let ident = spanned(text::ident().padded().map(String::from)).map(DataMatcher::Ident);

        // Parse arrays: type followed by []
        let arrayable = ident
            .or(object)
            .then(just("[]").padded().repeated().count())
            .map(|(base, brackets)| {
                (0..brackets).fold(base, |inner, _| DataMatcher::Array {
                    element: Box::new(inner),
                })
            });

        // Parse unions: A | B | C
        let union = arrayable
            .separated_by(just('|').padded())
            .at_least(1)
            .collect::<Vec<_>>()
            .map(|mut items: Vec<DataMatcher>| {
                if items.len() == 1 {
                    items.remove(0)
                } else {
                    DataMatcher::Union(items)
                }
            });

        union
    })
    .then_ignore(end())
}

pub fn parse(source: &str) -> Result<DataMatcher, Vec<Error<'_>>> {
    parser().parse(source).into_result()
}

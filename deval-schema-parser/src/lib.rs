use chumsky::prelude::*;
use chumsky::text;

use deval_schema_ast::Spanned;
use deval_schema_ast::{Expression, RecordMatcher};

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

fn parser<'a>() -> impl Parser<'a, &'a str, Expression, extra::Err<Error<'a>>> {
    recursive(|data| {
        // Parse doc comments (/// lines)
        let doc_comment = just("///")
            .ignore_then(none_of("\n").repeated().collect::<String>())
            .padded();

        // Parse a record field: docs + key + colon + data type
        let simple_key_record = doc_comment
            .repeated()
            .collect::<Vec<_>>()
            .map(|docs| docs.join("\n"))
            .then(text::ident().map(String::from).then(just("?").or_not()))
            .then_ignore(just(':').padded())
            .then(data.clone())
            .map(
                |((docs, (key, is_optional)), value)| RecordMatcher::SimpleKey {
                    key,
                    optional: is_optional.is_some(),
                    docs,
                    value,
                },
            );

        let any_key_record = just("..").padded().map(|_| RecordMatcher::AnyKey);
        let record = simple_key_record.or(any_key_record);

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
            .map(Expression::Object);

        // Parse basic identifiers (string, number, etc.)
        let ident = spanned(text::ident().padded().map(String::from)).map(Expression::Ident);
        let number = spanned(
            text::digits(10)
                .collect::<String>()
                .padded()
                .map(|x| x.parse().unwrap()),
        )
        .map(Expression::Number);

        let number_or_ident = number.or(ident);
        let range = spanned(number_or_ident.clone().map(Box::new))
            .or_not()
            .then_ignore(just(".."))
            .then(just("=").or_not())
            .then(spanned(number_or_ident.clone().map(Box::new)).or_not())
            .map(|x| Expression::Range {
                start: x.0.0,
                end: x.1,
                is_inclusive: x.0.1.is_some(),
            });

        let array_index = just("[")
            .padded()
            .ignore_then(spanned(data.map(Box::new)).or_not())
            .then_ignore(just("]").padded());

        // Parse arrays: type followed by []
        let arrayable = number_or_ident
            .or(range)
            .or(object)
            .then(array_index.padded().repeated().collect::<Vec<_>>())
            .map(|(base, brackets)| {
                brackets
                    .into_iter()
                    .fold(base, |inner, index| Expression::Array {
                        element: Box::new(inner),
                        index,
                    })
            });

        // Parse unions: A | B | C
        let union = arrayable
            .separated_by(just('|').padded())
            .at_least(1)
            .collect::<Vec<_>>()
            .map(|mut items: Vec<Expression>| {
                if items.len() == 1 {
                    items.remove(0)
                } else {
                    Expression::Union(items)
                }
            });

        union
    })
    .then_ignore(end())
}

pub fn parse(source: &str) -> Result<Expression, Vec<Error<'_>>> {
    parser().parse(source).into_result()
}

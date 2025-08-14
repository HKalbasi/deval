use std::collections::HashMap;

use deval_data_model::SpannedData;
use deval_schema_ast::DataMatcher;
pub use deval_schema_parser::Error;
use deval_schema_parser::SimpleSpan;
use deval_validator::{ArrayValidator, LambdaValidator, ObjectValidator, OrValidator, Validator};

fn compile_ast(
    ast: DataMatcher,
    env: &HashMap<String, Box<dyn Validator>>,
) -> Result<Box<dyn Validator>, Error<'static>> {
    match ast {
        DataMatcher::Ident(ident) => Ok(env
            .get(&ident.value)
            .ok_or_else(|| {
                Error::custom(
                    SimpleSpan {
                        start: ident.span.start,
                        end: ident.span.end,
                        context: (),
                    },
                    "Unknown ident",
                )
            })?
            .clone()),
        DataMatcher::Array { element } => Ok(Box::new(ArrayValidator(compile_ast(*element, env)?))),
        DataMatcher::Object(record_matchers) => Ok(Box::new(ObjectValidator(
            record_matchers
                .into_iter()
                .map(|r| Ok((r.key, r.docs, compile_ast(r.value, env)?)))
                .collect::<Result<_, _>>()?,
        ))),
        DataMatcher::Union(cases) => Ok(Box::new(OrValidator(
            cases
                .into_iter()
                .map(|x| compile_ast(x, env))
                .collect::<Result<_, _>>()?,
        ))),
    }
}

fn default_env() -> HashMap<String, Box<dyn Validator>> {
    let key_values: [(String, Box<dyn Validator>); _] = [
        (
            "string".to_owned(),
            Box::new(LambdaValidator(|d| {
                if !matches!(d.value, SpannedData::String(_)) {
                    Some(format!("Expected String, found {}", d.value.kind()))
                } else {
                    None
                }
            })),
        ),
        (
            "number".to_owned(),
            Box::new(LambdaValidator(|d| {
                if !matches!(d.value, SpannedData::Number(_)) {
                    Some(format!("Expected Number, found {}", d.value.kind()))
                } else {
                    None
                }
            })),
        ),
        (
            "integer".to_owned(),
            Box::new(LambdaValidator(|d| {
                if !matches!(&d.value, SpannedData::Number(n) if n.value.fract() == 0.) {
                    Some(format!("Expected Integer, found {}", d.value.kind()))
                } else {
                    None
                }
            })),
        ),
        (
            "null".to_owned(),
            Box::new(LambdaValidator(|d| {
                if !matches!(d.value, SpannedData::Null) {
                    Some(format!("Expected Null, found {}", d.value.kind()))
                } else {
                    None
                }
            })),
        ),
        (
            "bool".to_owned(),
            Box::new(LambdaValidator(|d| {
                if !matches!(d.value, SpannedData::Bool(_)) {
                    Some(format!("Expected Bool, found {}", d.value.kind()))
                } else {
                    None
                }
            })),
        ),
        ("any".to_owned(), Box::new(LambdaValidator(|_| None))),
    ];
    HashMap::from(key_values)
}

pub fn compile(source: &str) -> Result<Box<dyn Validator>, Vec<Error<'_>>> {
    let ast = deval_schema_parser::parse(source)?;
    Ok(compile_ast(ast, &default_env()).map_err(|e| vec![e])?)
}

use std::{collections::HashMap, ops::Range};

use deval_data_model::SpannedData;
use deval_schema_ast::Expression;
pub use deval_schema_parser::Error;
use deval_schema_parser::SimpleSpan;
use deval_validator::{
    ArrayValidator, LambdaValidator, ObjectValidator, OrValidator, RecordValidator, Validator,
};

#[derive(Clone)]
enum Value {
    Number(f64),
    Range {
        start: Option<f64>,
        end: Option<f64>,
        is_inclusive: bool,
    },
    Validator(Box<dyn Validator>),
}

impl Value {
    fn to_validator(self) -> Box<dyn Validator> {
        match self {
            Value::Number(_) => todo!(),
            Value::Range { .. } => todo!(),
            Value::Validator(validator) => validator,
        }
    }

    fn from_validator<T: Validator + 'static>(v: T) -> Self {
        Self::Validator(Box::new(v))
    }
}

fn eval_as_validator(
    ast: Expression,
    env: &HashMap<String, Value>,
) -> Result<Box<dyn Validator>, Error<'static>> {
    let value = compile_ast(ast, env)?;
    Ok(value.to_validator())
}

fn eval_as_number(ast: Expression, span: Range<usize>, env: &HashMap<String, Value>) -> Result<f64, Error<'static>> {
    let value = compile_ast(ast, env)?;
    match value {
        Value::Number(n) => Ok(n),
        _ => Err(Error::custom(
            SimpleSpan { start: span.start, end: span.end, context: () },
            "Unknown ident",
        )),
    }
}

fn compile_ast(ast: Expression, env: &HashMap<String, Value>) -> Result<Value, Error<'static>> {
    match ast {
        Expression::Number(x) => Ok(Value::Number(x.value)),
        Expression::Range {
            start,
            end,
            is_inclusive,
        } => {
            let start = match start {
                Some(x) => Some(eval_as_number(*x.value, x.span, env)?),
                None => None,
            };
            let end = match end {
                Some(x) => Some(eval_as_number(*x.value, x.span, env)?),
                None => None,
            };
            Ok(Value::Range {
                start,
                end,
                is_inclusive,
            })
        }
        Expression::Ident(ident) => Ok(env
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
        Expression::Array { element } => Ok(Value::from_validator(ArrayValidator(
            eval_as_validator(*element, env)?,
        ))),
        Expression::Object(record_matchers) => Ok(Value::from_validator(ObjectValidator(
            record_matchers
                .into_iter()
                .map(|r| {
                    Ok(match r {
                        deval_schema_ast::RecordMatcher::SimpleKey {
                            key,
                            docs,
                            value,
                            optional,
                        } => RecordValidator::SimpleKey {
                            key,
                            docs,
                            value: eval_as_validator(value, env)?,
                            optional,
                        },
                        deval_schema_ast::RecordMatcher::AnyKey => RecordValidator::AnyKey,
                    })
                })
                .collect::<Result<_, _>>()?,
        ))),
        Expression::Union(cases) => Ok(Value::from_validator(OrValidator(
            cases
                .into_iter()
                .map(|x| eval_as_validator(x, env))
                .collect::<Result<_, _>>()?,
        ))),
    }
}

fn default_env() -> HashMap<String, Value> {
    let key_values: [(String, Value); _] = [
        (
            "string".to_owned(),
            Value::from_validator(LambdaValidator(|d| {
                if !matches!(d.value, SpannedData::String(_)) {
                    Some(format!("Expected String, found {}", d.value.kind()))
                } else {
                    None
                }
            })),
        ),
        (
            "number".to_owned(),
            Value::from_validator(LambdaValidator(|d| {
                if !matches!(d.value, SpannedData::Number(_)) {
                    Some(format!("Expected Number, found {}", d.value.kind()))
                } else {
                    None
                }
            })),
        ),
        (
            "integer".to_owned(),
            Value::from_validator(LambdaValidator(|d| {
                if !matches!(&d.value, SpannedData::Number(n) if n.value.fract() == 0.) {
                    Some(format!("Expected Integer, found {}", d.value.kind()))
                } else {
                    None
                }
            })),
        ),
        (
            "null".to_owned(),
            Value::from_validator(LambdaValidator(|d| {
                if !matches!(d.value, SpannedData::Null) {
                    Some(format!("Expected Null, found {}", d.value.kind()))
                } else {
                    None
                }
            })),
        ),
        (
            "bool".to_owned(),
            Value::from_validator(LambdaValidator(|d| {
                if !matches!(d.value, SpannedData::Bool(_)) {
                    Some(format!("Expected Bool, found {}", d.value.kind()))
                } else {
                    None
                }
            })),
        ),
        (
            "any".to_owned(),
            Value::from_validator(LambdaValidator(|_| None)),
        ),
    ];
    HashMap::from(key_values)
}

pub fn compile(source: &str) -> Result<Box<dyn Validator>, Vec<Error<'_>>> {
    let ast = deval_schema_parser::parse(source)?;
    Ok(eval_as_validator(ast, &default_env()).map_err(|e| vec![e])?)
}

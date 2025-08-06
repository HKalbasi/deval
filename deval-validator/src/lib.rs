use deval_data_model::{Annotated, AnnotatedData, Span, Spanned, SpannedData};
use dyn_clone::DynClone;

pub struct ValidationError {
    pub span: Span,
    pub text: String,
}

pub struct ValidationResult {
    pub result: Annotated<AnnotatedData>,
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    fn ok(result: Annotated<AnnotatedData>) -> Self {
        Self {
            result,
            errors: vec![],
        }
    }

    fn append_errors_and_return_result(
        self,
        errors: &mut Vec<ValidationError>,
    ) -> Annotated<AnnotatedData> {
        errors.extend(self.errors);
        self.result
    }
}

pub trait Validator: std::fmt::Debug + DynClone + Send + Sync {
    fn validate(&self, data: Spanned<SpannedData>) -> ValidationResult;
}

dyn_clone::clone_trait_object!(Validator);

#[derive(Debug, Clone, Copy)]
pub struct AnyValidator;

impl Validator for AnyValidator {
    fn validate(&self, data: Spanned<SpannedData>) -> ValidationResult {
        ValidationResult::ok(data.into())
    }
}

#[derive(Clone)]
pub struct LambdaValidator<T: Clone + Fn(Spanned<SpannedData>) -> Option<String>>(pub T);

impl<T: Clone + Fn(Spanned<SpannedData>) -> Option<String>> std::fmt::Debug for LambdaValidator<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("LambdaValidator").finish()
    }
}

impl<T: Clone + Send + Sync + Fn(Spanned<SpannedData>) -> Option<String>> Validator for LambdaValidator<T> {
    fn validate(&self, data: Spanned<SpannedData>) -> ValidationResult {
        let span = data.span.clone();
        if let Some(text) = self.0(data.clone()) {
            return ValidationResult {
                errors: vec![ValidationError { span, text }],
                result: data.into(),
            };
        } else {
            ValidationResult::ok(data.into())
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NumberValidator;

impl Validator for NumberValidator {
    fn validate(&self, data: Spanned<SpannedData>) -> ValidationResult {
        let SpannedData::Number(_n) = &data.value else {
            return ValidationResult {
                errors: vec![ValidationError {
                    span: data.span.clone(),
                    text: format!("Expected Number, found {}", data.value.kind()),
                }],
                result: data.into(),
            };
        };
        ValidationResult::ok(data.into())
    }
}

#[derive(Debug, Clone)]
pub struct ArrayValidator(pub Box<dyn Validator>);

impl Validator for ArrayValidator {
    fn validate(&self, data: Spanned<SpannedData>) -> ValidationResult {
        let SpannedData::Array(items) = data.value else {
            return ValidationResult {
                errors: vec![ValidationError {
                    span: data.span.clone(),
                    text: format!("Expected Object, found {}", data.value.kind()),
                }],
                result: data.into(),
            };
        };
        let mut errors = vec![];
        let items = items
            .into_iter()
            .map(|x| {
                self.0
                    .validate(x)
                    .append_errors_and_return_result(&mut errors)
            })
            .collect();
        ValidationResult {
            result: Annotated {
                value: AnnotatedData::Array(items),
                span: data.span,
                docs: String::new(),
                semantic_type: None,
            },
            errors,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjectValidator(pub Vec<(String, Box<dyn Validator>)>);

impl Validator for ObjectValidator {
    fn validate(&self, data: Spanned<SpannedData>) -> ValidationResult {
        let SpannedData::Object(key_values) = data.value else {
            return ValidationResult {
                errors: vec![ValidationError {
                    span: data.span.clone(),
                    text: format!("Expected Object, found {}", data.value.kind()),
                }],
                result: data.into(),
            };
        };
        let mut errors = vec![];
        let mut result: Vec<(Annotated<String>, Annotated<AnnotatedData>)> = vec![];
        for (key, value) in key_values {
            let Some((_, validator)) = self.0.iter().find(|x| x.0 == key.value) else {
                errors.push(ValidationError {
                    span: key.span,
                    text: format!("Unexpected key {}", key.value),
                });
                continue;
            };
            let r = validator.validate(value);
            result.push((key.into(), r.append_errors_and_return_result(&mut errors)));
        }
        ValidationResult {
            result: Annotated {
                value: AnnotatedData::Object(result),
                span: data.span,
                docs: String::new(),
                semantic_type: None,
            },
            errors,
        }
    }
}

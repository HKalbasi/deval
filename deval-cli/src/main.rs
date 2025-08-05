use ariadne::{Label, Report, ReportKind, Source};
use deval_validator::{ArrayValidator, NumberValidator, ObjectValidator, ValidationError, ValidationResult, Validator};
use std::ops::Range;
use tree_sitter::{Node, Parser};

use deval_data_model::{Span, Spanned, SpannedData};

fn parse_json_with_spans(
    filename: &str,
    source: &str,
) -> Result<SpannedData, Vec<(String, Range<usize>)>> {
    // Initialize tree-sitter JSON parser
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .unwrap();

    let tree = parser.parse(source, None).unwrap();
    let root_node = tree.root_node();

    println!("{}", root_node.to_sexp());

    let mut errors = Vec::new();
    let result = parse_value(&root_node, source, filename, &mut errors);

    if !errors.is_empty() {
        Err(errors)
    } else {
        result.ok_or_else(|| vec![])
    }
}

fn parse_value(
    node: &Node,
    source: &str,
    filename: &str,
    errors: &mut Vec<(String, Range<usize>)>,
) -> Option<SpannedData> {
    match node.kind() {
        "null" => Some(SpannedData::Null),
        "false" | "true" => Some(SpannedData::Bool(Spanned {
            value: node.kind() == "true",
            span: make_span(node, filename),
        })),
        "number" => {
            let text = node.utf8_text(source.as_bytes()).ok()?;
            match text.parse::<f64>() {
                Ok(num) => Some(SpannedData::Number(Spanned {
                    value: num,
                    span: make_span(node, filename),
                })),
                Err(e) => {
                    errors.push((
                        format!("Failed to parse number '{}': {}", text, e),
                        node.byte_range(),
                    ));
                    None
                }
            }
        }
        "string" => {
            let text = node.utf8_text(source.as_bytes()).ok()?;
            // Remove quotes
            let content = text[1..text.len() - 1].to_string();
            Some(SpannedData::String(Spanned {
                value: content,
                span: make_span(node, filename),
            }))
        }
        "array" => {
            let mut children = Vec::new();
            let mut cursor = node.walk();

            for child in node.children(&mut cursor) {
                if ["[", ",", "]"].contains(&child.kind()) {
                    continue;
                }
                let value = parse_value(&child, source, filename, errors)?;
                children.push(Spanned {
                    value,
                    span: make_span(&child, filename),
                });
            }

            Some(SpannedData::Array(children))
        }
        "object" => {
            let mut pairs = Vec::new();
            let mut cursor = node.walk();

            for child in node.children(&mut cursor) {
                match child.kind() {
                    "pair" => {
                        let key_node = child
                            .child_by_field_name("key")
                            .or_else(|| child.named_child(0))?;
                        let value_node = child
                            .child_by_field_name("value")
                            .or_else(|| child.named_child(1))?;
                        let key = parse_string_value(&key_node, source, filename, errors)?;
                        let value = parse_value(&value_node, source, filename, errors)?;
                        pairs.push((
                            Spanned {
                                value: key,
                                span: make_span(&key_node, filename),
                            },
                            Spanned {
                                value,
                                span: make_span(&value_node, filename),
                            },
                        ));
                    }
                    "{" | "," | "}" => (),
                    "ERROR" => {
                        errors.push((format!("Failed to parse json:"), child.byte_range()));
                        return None;
                    }
                    _ => {
                        errors.push((
                            format!("Unexpected node type: {}", child.kind()),
                            child.byte_range(),
                        ));
                        return None;
                    }
                }
            }

            Some(SpannedData::Object(pairs))
        }
        "document" => parse_value(&node.child(0).unwrap(), source, filename, errors),
        _ => {
            errors.push((
                format!("Unexpected node type: {}", node.kind()),
                node.byte_range(),
            ));
            None
        }
    }
}

fn parse_string_value(
    node: &Node,
    source: &str,
    filename: &str,
    errors: &mut Vec<(String, Range<usize>)>,
) -> Option<String> {
    if node.kind() != "string" {
        errors.push((
            format!("Expected string, got {}", node.kind()),
            node.byte_range(),
        ));
        return None;
    }

    let text = node.utf8_text(source.as_bytes()).ok()?;
    // Remove quotes
    Some(text[1..text.len() - 1].to_string())
}

fn make_span(node: &Node, filename: &str) -> Span {
    Span {
        filename: filename.to_string(),
        start: node.start_byte(),
        end: node.end_byte(),
    }
}

fn report_validation_errors(source: &str, errors: &[ValidationError]) {
    for error in errors {
        let source = Source::from(source);
        // Create a simple error report pointing to the beginning of the file
        // In a real implementation, you'd want to map errors to specific positions
        let filename = &*error.span.filename;
        let span = error.span.start..error.span.end;
        Report::build(ReportKind::Error, (filename, span.clone()))
            .with_message(&error.text)
            .with_label(Label::new((filename, span.clone())).with_message("error occurred here"))
            .finish()
            .print((filename, source))
            .unwrap();
    }
}

fn report_errors(filename: &str, source: &str, errors: &[(String, Range<usize>)]) {
    for (error, span) in errors {
        let source = Source::from(source);
        // Create a simple error report pointing to the beginning of the file
        // In a real implementation, you'd want to map errors to specific positions
        Report::build(ReportKind::Error, (filename, span.clone()))
            .with_message(error)
            .with_label(Label::new((filename, span.clone())).with_message("error occurred here"))
            .finish()
            .print((filename, source))
            .unwrap();
    }
}

fn main() {
    let filename = "example.json";
    let source = r#"
    {
        "name": "John Doe",
        "age": 30,
        "is_student": false,
        "address": {
            "street": "123 Main St",
            "city": "Anytown"
        },
        "hobbies": ["reading", "swimming"]
    }
    "#;

    match parse_json_with_spans(filename, source) {
        Ok(data) => {
            println!("Parsed successfully: {:#?}", data);
            let validator = ObjectValidator(vec![
                ("name".to_owned(), Box::new(NumberValidator)),
                ("hobbies".to_owned(), Box::new(ArrayValidator(Box::new(NumberValidator)))),
                ("address".to_owned(), Box::new(ObjectValidator(vec![]))),
            ]);
            let r = validator.validate(Spanned {
                value: data,
                span: Span {
                    filename: "()".to_string(),
                    start: 0,
                    end: 1,
                },
            });
            println!("Parsed successfully: {:#?}", r.result);
            report_validation_errors(source, &r.errors);
        }
        Err(errors) => {
            eprintln!("Failed to parse JSON:");
            report_errors(filename, source, &errors);
        }
    }
}

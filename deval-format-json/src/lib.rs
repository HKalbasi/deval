use deval_data_model::{Format, ParseError, Span, Spanned, SpannedData};
use tree_sitter::{Node, Parser};

pub struct Json;

impl Format for Json {
    fn parse(&self, source: &str, filename: &str) -> Result<Spanned<SpannedData>, Vec<ParseError>> {
        // Initialize tree-sitter JSON parser
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_json::language())
            .unwrap();

        let tree = parser.parse(source, None).unwrap();
        let root_node = tree.root_node();

        let mut errors = Vec::new();
        let result = parse_value(&root_node, source, filename, &mut errors);

        let result = result.map(|x| Spanned {
            value: x,
            span: Span {
                filename: "()".to_owned(),
                start: 0,
                end: 1,
            },
        });

        if !errors.is_empty() {
            Err(errors)
        } else {
            result.ok_or_else(|| vec![])
        }
    }
}

fn parse_value(
    node: &Node,
    source: &str,
    filename: &str,
    errors: &mut Vec<ParseError>,
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
                    errors.push(ParseError {
                        message: format!("Failed to parse number '{}': {}", text, e),
                        span: make_span(node, filename),
                    });
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
                        let key = parse_string_value(&key_node, source, errors)?;
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
                        errors.push(ParseError {
                            message: format!("Failed to parse json:"),
                            span: make_span(&child, filename),
                        });
                        return None;
                    }
                    _ => {
                        errors.push(ParseError {
                            message: format!("Unexpected node type: {}", child.kind()),
                            span: make_span(&child, dbg!(filename)),
                        });
                        return None;
                    }
                }
            }

            Some(SpannedData::Object(pairs))
        }
        "document" => parse_value(&node.child(0).unwrap(), source, filename, errors),
        _ => {
            errors.push(ParseError {
                message: format!("Unexpected node type: {}", node.kind()),
                span: make_span(&node, dbg!(filename)),
            });
            None
        }
    }
}

fn parse_string_value(
    node: &Node,
    source: &str,
    errors: &mut Vec<ParseError>,
) -> Option<String> {
    if node.kind() != "string" {
        errors.push(ParseError {
            message: format!("Expected string, got {}", node.kind()),
            span: make_span(node, "foo"),
        });
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

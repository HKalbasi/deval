use deval_data_model::{Format, ParseError, Span, SpanSet, Spanned, SpannedData};
use tree_sitter::{Node, Parser};

pub struct Json;

impl Format for Json {
    fn parse(&self, source: &str, filename: &str) -> Result<Spanned<SpannedData>, Vec<ParseError>> {
        // Initialize tree-sitter JSON parser
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_json::language()).unwrap();

        let tree = parser.parse(source, None).unwrap();
        let root_node = tree.root_node();

        let mut errors = Vec::new();
        let result = parse_value(&root_node, source, filename, &mut errors);

        let result = result.map(|x| Spanned {
            value: x,
            annotation: make_span_vec(&root_node, filename),
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
            annotation: make_span_vec(node, filename),
        })),
        "number" => {
            let text = node.utf8_text(source.as_bytes()).ok()?;
            match text.parse::<f64>() {
                Ok(num) => Some(SpannedData::Number(Spanned {
                    value: num,
                    annotation: make_span_vec(node, filename),
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
                annotation: make_span_vec(node, filename),
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
                    annotation: make_span_vec(&child, filename),
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
                                annotation: make_span_vec(&key_node, filename),
                            },
                            Spanned {
                                value,
                                annotation: make_span_vec(&value_node, filename),
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
                            span: make_span(&child, filename),
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
                span: make_span(&node, filename),
            });
            None
        }
    }
}

fn parse_string_value(node: &Node, source: &str, errors: &mut Vec<ParseError>) -> Option<String> {
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

/// Creates a `Span` from a `tree_sitter::Node`.
fn make_span(node: &Node, filename: &str) -> Span {
    Span {
        filename: filename.to_string(),
        start: node.start_byte(),
        end: node.end_byte(),
    }
}

/// Creates a `Vec<Span>` from a `tree_sitter::Node`.
fn make_span_vec(node: &Node, filename: &str) -> SpanSet {
    SpanSet(vec![make_span(node, filename)])
}

#[cfg(test)]
mod tests {
    use super::*;
    use deval_data_model::{Format, SpannedData};

    #[test]
    fn test_parse_simple_object() {
        let json = r#"{"name": "John", "age": 30}"#;
        let result = Json.parse(json, "test.json");

        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse JSON");

        match parsed.value {
            SpannedData::Object(pairs) => {
                assert_eq!(pairs.len(), 2);

                // Check name field
                assert_eq!(pairs[0].0.value, "name");
                match &pairs[0].1.value {
                    SpannedData::String(s) => assert_eq!(s.value, "John"),
                    _ => panic!("Expected string value for name"),
                }

                // Check age field
                assert_eq!(pairs[1].0.value, "age");
                match &pairs[1].1.value {
                    SpannedData::Number(n) => assert_eq!(n.value, 30.0),
                    _ => panic!("Expected number value for age"),
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_parse_array() {
        let json = r#"[1, 2, 3]"#;
        let result = Json.parse(json, "test.json");

        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse JSON");

        match parsed.value {
            SpannedData::Array(items) => {
                assert_eq!(items.len(), 3);
                for (i, item) in items.iter().enumerate() {
                    match &item.value {
                        SpannedData::Number(n) => assert_eq!(n.value, (i + 1) as f64),
                        _ => panic!("Expected number values in array"),
                    }
                }
            }
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_parse_nested_object() {
        let json = r#"{"person": {"name": "Alice", "age": 25}}"#;
        let result = Json.parse(json, "test.json");

        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse JSON");

        match parsed.value {
            SpannedData::Object(pairs) => {
                assert_eq!(pairs.len(), 1);
                assert_eq!(pairs[0].0.value, "person");

                match &pairs[0].1.value {
                    SpannedData::Object(inner_pairs) => {
                        assert_eq!(inner_pairs.len(), 2);

                        // Check name field
                        assert_eq!(inner_pairs[0].0.value, "name");
                        match &inner_pairs[0].1.value {
                            SpannedData::String(s) => assert_eq!(s.value, "Alice"),
                            _ => panic!("Expected string value for name"),
                        }

                        // Check age field
                        assert_eq!(inner_pairs[1].0.value, "age");
                        match &inner_pairs[1].1.value {
                            SpannedData::Number(n) => assert_eq!(n.value, 25.0),
                            _ => panic!("Expected number value for age"),
                        }
                    }
                    _ => panic!("Expected nested object"),
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_parse_boolean_and_null() {
        let json = r#"{"active": true, "deleted": false, "value": null}"#;
        let result = Json.parse(json, "test.json");

        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse JSON");

        match parsed.value {
            SpannedData::Object(pairs) => {
                assert_eq!(pairs.len(), 3);

                // Check active field (true)
                assert_eq!(pairs[0].0.value, "active");
                match &pairs[0].1.value {
                    SpannedData::Bool(b) => assert_eq!(b.value, true),
                    _ => panic!("Expected boolean true"),
                }

                // Check deleted field (false)
                assert_eq!(pairs[1].0.value, "deleted");
                match &pairs[1].1.value {
                    SpannedData::Bool(b) => assert_eq!(b.value, false),
                    _ => panic!("Expected boolean false"),
                }

                // Check value field (null)
                assert_eq!(pairs[2].0.value, "value");
                match &pairs[2].1.value {
                    SpannedData::Null => {} // Correct
                    _ => panic!("Expected null value"),
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_parse_float_number() {
        let json = r#"{"price": 19.99}"#;
        let result = Json.parse(json, "test.json");

        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse JSON");

        match parsed.value {
            SpannedData::Object(pairs) => {
                assert_eq!(pairs.len(), 1);
                assert_eq!(pairs[0].0.value, "price");

                match &pairs[0].1.value {
                    SpannedData::Number(n) => assert_eq!(n.value, 19.99),
                    _ => panic!("Expected number value"),
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_parse_empty_object() {
        let json = r#"{}"#;
        let result = Json.parse(json, "test.json");

        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse JSON");

        match parsed.value {
            SpannedData::Object(pairs) => {
                assert_eq!(pairs.len(), 0);
            }
            _ => panic!("Expected empty object"),
        }
    }

    #[test]
    fn test_parse_empty_array() {
        let json = r#"[]"#;
        let result = Json.parse(json, "test.json");

        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse JSON");

        match parsed.value {
            SpannedData::Array(items) => {
                assert_eq!(items.len(), 0);
            }
            _ => panic!("Expected empty array"),
        }
    }

    #[test]
    fn test_parse_invalid_json() {
        let json = r#"{"name": "John", "age": 30,}"#; // Trailing comma not allowed in JSON
        let result = Json.parse(json, "test.json");

        // This should fail
        assert!(result.is_err());
    }
}

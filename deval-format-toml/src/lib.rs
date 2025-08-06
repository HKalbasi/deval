use deval_data_model::{Format, ParseError, Span, Spanned, SpannedData};
use tree_sitter::{Node, Parser};

pub struct Toml;

impl Format for Toml {
    fn parse(&self, source: &str, filename: &str) -> Result<Spanned<SpannedData>, Vec<ParseError>> {
        // Initialize tree-sitter TOML parser
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_toml::language())
            .unwrap();

        let tree = parser.parse(source, None).unwrap();
        let root_node = tree.root_node();

        let mut errors = Vec::new();

        // Check for top-level parsing errors
        if root_node.has_error() {
             errors.push(ParseError {
                message: "Failed to parse TOML structure.".to_string(),
                span: make_span(&root_node, filename),
            });
            // Attempt to find the specific error node for a better message, but this is a good fallback.
        }

        let result = parse_value(&root_node, source, filename, &mut errors);

        // Wrap the final result in a Spanned object covering the whole file
        let result = result.map(|x| Spanned {
            value: x,
            span: make_span(&root_node, filename),
        });

        if !errors.is_empty() {
            Err(errors)
        } else {
            // If there were no hard errors but parsing still failed, return an empty error vec.
            result.ok_or_else(Vec::new)
        }
    }
}

/// Recursively parses a tree-sitter node into SpannedData.
fn parse_value(
    node: &Node,
    source: &str,
    filename: &str,
    errors: &mut Vec<ParseError>,
) -> Option<SpannedData> {
    if node.is_error() {
        errors.push(ParseError {
            message: format!("Syntax error in TOML file."),
            span: make_span(node, filename),
        });
        return None;
    }

    match node.kind() {
        // Primitives
        "string" => {
            let text = node.utf8_text(source.as_bytes()).unwrap();
            // Simple unquoting. A full implementation would handle all escape sequences.
            let content = unquote_toml_string(text);
            Some(SpannedData::String(Spanned {
                value: content,
                span: make_span(node, filename),
            }))
        }
        "integer" | "float" => {
            let text = node.utf8_text(source.as_bytes()).unwrap();
            // TOML integers can have underscores, which Rust's parse handles.
            match text.replace('_', "").parse::<f64>() {
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
        "boolean" => Some(SpannedData::Bool(Spanned {
            value: node.utf8_text(source.as_bytes()).unwrap() == "true",
            span: make_span(node, filename),
        })),
        "date_time" => {
            // For simplicity, treat dates as strings.
            let text = node.utf8_text(source.as_bytes()).unwrap().to_string();
            Some(SpannedData::String(Spanned {
                value: text,
                span: make_span(node, filename),
            }))
        }

        // Collections
        "array" => {
            let mut children = Vec::new();
            let mut cursor = node.walk();

            // Use named_children to automatically skip syntax like `[` `,` `]`
            for child in node.named_children(&mut cursor) {
                if let Some(value) = parse_value(&child, source, filename, errors) {
                    children.push(Spanned {
                        value,
                        span: make_span(&child, filename),
                    });
                }
            }
            Some(SpannedData::Array(children))
        }

        // Object-like structures
        "document" | "table" | "inline_table" => {
            let mut pairs = Vec::new();
            let mut cursor = node.walk();

            for child in node.children(&mut cursor) {
                match child.kind() {
                    "pair" => {
                        let key_node = child.child(0).unwrap();
                        let value_node = child.child(2).unwrap();

                        let key_text = key_node.utf8_text(source.as_bytes()).unwrap().to_string();
                        let value_data = parse_value(&value_node, source, filename, errors)?;

                        pairs.push((
                            Spanned {
                                value: key_text,
                                span: make_span(&key_node, filename),
                            },
                            Spanned {
                                value: value_data,
                                span: make_span(&value_node, filename),
                            },
                        ));
                    }
                    "table" => {
                        // A table defined inside another table, e.g., [a] then [a.b]
                        // This parser creates nested objects based on the tree structure.
                        let key_node = child.child(1).unwrap();
                        let key_text = key_node.utf8_text(source.as_bytes()).unwrap().to_string();
                        
                        // The table node itself contains its pairs, so parse it recursively as an object.
                        let table_as_object = parse_value(&child, source, filename, errors)?;

                        pairs.push((
                            Spanned {
                                value: key_text,
                                span: make_span(&key_node, filename),
                            },
                            Spanned {
                                value: table_as_object,
                                span: make_span(&child, filename),
                            },
                        ));
                    }
                    // Ignore comments, newlines, and other non-data nodes
                    _ => continue,
                }
            }
            Some(SpannedData::Object(pairs))
        }

        // Default case for unexpected nodes
        _ => {
            errors.push(ParseError {
                message: format!("Unexpected TOML node type: {}", node.kind()),
                span: make_span(&node, filename),
            });
            None
        }
    }
}

/// Creates a `Span` from a `tree_sitter::Node`.
fn make_span(node: &Node, filename: &str) -> Span {
    Span {
        filename: filename.to_string(),
        start: node.start_byte(),
        end: node.end_byte(),
    }
}

/// A simple helper to remove quotes from TOML string literals.
/// Does not handle escape sequences.
fn unquote_toml_string(text: &str) -> String {
    if text.starts_with("\"\"\"") && text.ends_with("\"\"\"") {
        return text[3..text.len() - 3].to_string();
    }
    if text.starts_with("'''") && text.ends_with("'''") {
        return text[3..text.len() - 3].to_string();
    }
    if text.starts_with('"') && text.ends_with('"') {
        return text[1..text.len() - 1].to_string();
    }
    if text.starts_with('\'') && text.ends_with('\'') {
        return text[1..text.len() - 1].to_string();
    }
    text.to_string()
}

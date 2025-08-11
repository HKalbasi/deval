use deval_data_model::{Format, ParseError, Span, SpanSet, Spanned, SpannedData};
use tree_sitter::{Node, Parser};

pub struct Toml;

impl Format for Toml {
    fn parse(&self, source: &str, filename: &str) -> Result<Spanned<SpannedData>, Vec<ParseError>> {
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_toml::language())
            .expect("Error loading TOML grammar");

        let tree = parser.parse(source, None).unwrap();
        let root_node = tree.root_node();

        let mut errors = Vec::new();

        if root_node.has_error() {
            errors.push(ParseError {
                message: "Failed to parse TOML structure due to syntax errors.".to_string(),
                span: make_span(&root_node, filename),
            });
        }

        let mut root_data = SpannedData::Object(Vec::new());

        // Iterate through all top-level nodes in the document.
        let mut cursor = root_node.walk();
        for node in root_node.children(&mut cursor) {
            match node.kind() {
                // A key-value pair at the top level.
                "pair" => {
                    if let SpannedData::Object(pairs) = &mut root_data {
                        if let Some((key, value)) = parse_pair(&node, source, filename, &mut errors)
                        {
                            if pairs.iter().any(|(k, _)| k.value == key.value) {
                                errors.push(ParseError {
                                    message: format!("Duplicate key '{}' at top level", key.value),
                                    span: key.annotation.primary(),
                                });
                            } else {
                                pairs.push((key, value));
                            }
                        }
                    }
                }
                // A standard table like `[table]`.
                "table" => {
                    // The key is the second child: '[' -> key -> ']'
                    let key_node = match node.child(1) {
                        Some(n) => n,
                        None => {
                            errors.push(ParseError {
                                message: "Table without a name".to_string(),
                                span: make_span(&node, filename),
                            });
                            continue;
                        }
                    };
                    let key_path = key_node.utf8_text(source.as_bytes()).unwrap();
                    let key_parts: Vec<&str> = key_path.split('.').collect();

                    // Get the target table, creating it if it doesn't exist.
                    if let Some(target_pairs) = get_or_insert_table(
                        &mut root_data,
                        &key_parts,
                        &node,
                        filename,
                        &mut errors,
                    ) {
                        // Now, parse all pairs that are *children* of this table node.
                        let mut table_cursor = node.walk();
                        for child in node.children(&mut table_cursor) {
                            if child.kind() == "pair" {
                                if let Some((key, value)) =
                                    parse_pair(&child, source, filename, &mut errors)
                                {
                                    if target_pairs.iter().any(|(k, _)| k.value == key.value) {
                                        errors.push(ParseError {
                                            message: format!(
                                                "Duplicate key '{}' in table '{}'",
                                                key.value, key_path
                                            ),
                                            span: key.annotation.primary(),
                                        });
                                    } else {
                                        target_pairs.push((key, value));
                                    }
                                }
                            }
                        }
                    }
                }
                // An array of tables like `[[array]]`.
                "table_array_element" => {
                    // The key is the second child: '[[' -> key -> ']]'
                    let key_node = match node.child(1) {
                        Some(n) => n,
                        None => {
                            errors.push(ParseError {
                                message: "Array table without a name".to_string(),
                                span: make_span(&node, filename),
                            });
                            continue;
                        }
                    };
                    let key_path = key_node.utf8_text(source.as_bytes()).unwrap();
                    let key_parts: Vec<&str> = key_path.split('.').collect();

                    // Append a new table to the array and get a reference to its pairs.
                    if let Some(target_pairs) = append_to_array_of_tables(
                        &mut root_data,
                        &key_parts,
                        &node,
                        filename,
                        &mut errors,
                    ) {
                        // Parse all pairs that are *children* of this array table node.
                        let mut array_table_cursor = node.walk();
                        for child in node.children(&mut array_table_cursor) {
                            if child.kind() == "pair" {
                                if let Some((key, value)) =
                                    parse_pair(&child, source, filename, &mut errors)
                                {
                                    if target_pairs.iter().any(|(k, _)| k.value == key.value) {
                                        errors.push(ParseError {
                                            message: format!(
                                                "Duplicate key '{}' in table '{}'",
                                                key.value, key_path
                                            ),
                                            span: key.annotation.primary(),
                                        });
                                    } else {
                                        target_pairs.push((key, value));
                                    }
                                }
                            }
                        }
                    }
                }
                // Ignore comments, newlines, etc.
                "comment" | "\n" => {}
                _ => {
                    if !node.is_extra() {
                        errors.push(ParseError {
                            message: format!("Unexpected top-level TOML node: {}", node.kind()),
                            span: make_span(&node, filename),
                        });
                    }
                }
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Spanned {
                value: root_data,
                annotation: make_span_vec(&root_node, filename),
            })
        }
    }
}

/// Navigates or creates a path of tables and returns a mutable reference to the final table's pairs.
/// Merges spans along the way.
fn get_or_insert_table<'a>(
    mut current_data: &'a mut SpannedData,
    path: &[&str],
    table_header_node: &Node,
    filename: &str,
    errors: &mut Vec<ParseError>,
) -> Option<&'a mut Vec<(Spanned<String>, Spanned<SpannedData>)>> {
    for (i, &key) in path.iter().enumerate() {
        let current_table_pairs = match current_data {
            SpannedData::Object(pairs) => pairs,
            _ => {
                errors.push(ParseError {
                    message: format!("Cannot define table '{}' because a key with this name was already defined as a non-table.", path[..i].join(".")),
                    span: make_span(table_header_node, filename),
                });
                return None;
            }
        };

        let found_index = current_table_pairs.iter().position(|(k, _)| k.value == key);

        if let Some(index) = found_index {
            let (found_key, found_value) = &mut current_table_pairs[index];
            // Don't add a span if it's for an implicitly created table.
            if !table_header_node.is_extra() {
                found_key
                    .annotation
                    .0
                    .push(make_span(table_header_node, filename));
                found_value
                    .annotation
                    .0
                    .push(make_span(table_header_node, filename));
            }
            current_data = &mut found_value.value;
        } else {
            let new_table = SpannedData::Object(Vec::new());
            let new_spanned_table = Spanned {
                value: new_table,
                annotation: make_span_vec(table_header_node, filename),
            };
            let new_spanned_key = Spanned {
                value: key.to_string(),
                annotation: make_span_vec(table_header_node, filename),
            };

            current_table_pairs.push((new_spanned_key, new_spanned_table));
            current_data = &mut current_table_pairs.last_mut().unwrap().1.value;
        }
    }

    if let SpannedData::Object(pairs) = current_data {
        Some(pairs)
    } else {
        errors.push(ParseError {
            message: format!("Cannot define table '{}' because a key with this name was already defined as a non-table.", path.join(".")),
            span: make_span(table_header_node, filename),
        });
        None
    }
}

/// Finds or creates an array of tables at the specified path and returns a mutable reference
/// to a new table element appended to it.
fn append_to_array_of_tables<'a>(
    current_data: &'a mut SpannedData,
    path: &[&str],
    array_header_node: &Node,
    filename: &str,
    errors: &mut Vec<ParseError>,
) -> Option<&'a mut Vec<(Spanned<String>, Spanned<SpannedData>)>> {
    let (array_key, table_path) = path.split_last()?;

    let parent_table = get_or_insert_table(
        current_data,
        table_path,
        array_header_node,
        filename,
        errors,
    )?;

    let found_index = parent_table.iter().position(|(k, _)| k.value == *array_key);

    let array = match found_index {
        Some(index) => {
            let (key, spanned_value) = &mut parent_table[index];
            key.annotation
                .0
                .push(make_span(array_header_node, filename));
            spanned_value
                .annotation
                .0
                .push(make_span(array_header_node, filename));
            if let SpannedData::Array(arr) = &mut spanned_value.value {
                arr
            } else {
                errors.push(ParseError {
                    message: format!("Key '{}' was already defined as a non-array.", array_key),
                    span: make_span(array_header_node, filename),
                });
                return None;
            }
        }
        None => {
            parent_table.push((
                Spanned {
                    value: array_key.to_string(),
                    annotation: make_span_vec(array_header_node, filename),
                },
                Spanned {
                    value: SpannedData::Array(Vec::new()),
                    annotation: make_span_vec(array_header_node, filename),
                },
            ));
            if let SpannedData::Array(arr) = &mut parent_table.last_mut().unwrap().1.value {
                arr
            } else {
                unreachable!()
            }
        }
    };

    array.push(Spanned {
        value: SpannedData::Object(Vec::new()),
        annotation: make_span_vec(array_header_node, filename),
    });

    if let Some(Spanned {
        value: SpannedData::Object(pairs),
        ..
    }) = array.last_mut()
    {
        Some(pairs)
    } else {
        None
    }
}

/// Parses a single key-value pair node.
fn parse_pair(
    pair_node: &Node,
    source: &str,
    filename: &str,
    errors: &mut Vec<ParseError>,
) -> Option<(Spanned<String>, Spanned<SpannedData>)> {
    // A `pair` node's children are `key`, `=`, `value`. We access by index.
    let key_node = pair_node.child(0)?;
    let value_node = pair_node.child(2)?;

    let key_text = unquote_toml_string(&key_node.utf8_text(source.as_bytes()).ok()?);
    let value_data = parse_value(&value_node, source, filename, errors)?;

    Some((
        Spanned {
            value: key_text,
            annotation: make_span_vec(&key_node, filename),
        },
        Spanned {
            value: value_data,
            annotation: make_span_vec(&value_node, filename),
        },
    ))
}

/// Recursively parses a tree-sitter node representing a VALUE into SpannedData.
fn parse_value(
    node: &Node,
    source: &str,
    filename: &str,
    errors: &mut Vec<ParseError>,
) -> Option<SpannedData> {
    if node.is_error() {
        errors.push(ParseError {
            message: "Syntax error in value.".to_string(),
            span: make_span(node, filename),
        });
        return None;
    }

    match node.kind() {
        "string" => {
            let text = node.utf8_text(source.as_bytes()).unwrap();
            let content = unquote_toml_string(text);
            Some(SpannedData::String(Spanned {
                value: content,
                annotation: make_span_vec(node, filename),
            }))
        }
        "integer" | "float" => {
            let text = node.utf8_text(source.as_bytes()).unwrap();
            match text.replace('_', "").parse::<f64>() {
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
        "boolean" => Some(SpannedData::Bool(Spanned {
            value: node.utf8_text(source.as_bytes()).unwrap() == "true",
            annotation: make_span_vec(node, filename),
        })),
        "date_time" => {
            let text = node.utf8_text(source.as_bytes()).unwrap().to_string();
            Some(SpannedData::String(Spanned {
                value: text,
                annotation: make_span_vec(node, filename),
            }))
        }
        "array" => {
            let mut children = Vec::new();
            let mut cursor = node.walk();
            for child in node.named_children(&mut cursor) {
                if let Some(value) = parse_value(&child, source, filename, errors) {
                    children.push(Spanned {
                        value,
                        annotation: make_span_vec(&child, filename),
                    });
                }
            }
            Some(SpannedData::Array(children))
        }
        "inline_table" => {
            let mut pairs = Vec::new();
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "pair" {
                    if let Some(pair) = parse_pair(&child, source, filename, errors) {
                        pairs.push(pair);
                    }
                }
            }
            Some(SpannedData::Object(pairs))
        }
        _ => {
            errors.push(ParseError {
                message: format!("Unexpected TOML value node type: {}", node.kind()),
                span: make_span(node, filename),
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

/// Creates a `Vec<Span>` from a `tree_sitter::Node`.
fn make_span_vec(node: &Node, filename: &str) -> SpanSet {
    SpanSet(vec![make_span(node, filename)])
}

/// A simple helper to remove quotes from TOML string literals.
/// Also handles bare keys.
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

#[cfg(test)]
mod tests {
    use super::*;
    use deval_data_model::{Format, SpannedData};

    #[test]
    fn test_parse_simple_key_value() {
        let toml = r#"name = "John Doe""#;
        let result = Toml.parse(toml, "test.toml");

        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse TOML");

        match parsed.value {
            SpannedData::Object(pairs) => {
                assert_eq!(pairs.len(), 1);
                assert_eq!(pairs[0].0.value, "name");
                match &pairs[0].1.value {
                    SpannedData::String(s) => assert_eq!(s.value, "John Doe"),
                    _ => panic!("Expected string value"),
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_parse_numbers() {
        let toml = r#"age = 30
height = 5.9"#;
        let result = Toml.parse(toml, "test.toml");

        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse TOML");

        match parsed.value {
            SpannedData::Object(pairs) => {
                assert_eq!(pairs.len(), 2);

                // Check age
                assert_eq!(pairs[0].0.value, "age");
                match &pairs[0].1.value {
                    SpannedData::Number(n) => assert_eq!(n.value, 30.0),
                    _ => panic!("Expected number value for age"),
                }

                // Check height
                assert_eq!(pairs[1].0.value, "height");
                match &pairs[1].1.value {
                    SpannedData::Number(n) => assert_eq!(n.value, 5.9),
                    _ => panic!("Expected number value for height"),
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_parse_booleans() {
        let toml = r#"is_active = true
is_deleted = false"#;
        let result = Toml.parse(toml, "test.toml");

        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse TOML");

        match parsed.value {
            SpannedData::Object(pairs) => {
                assert_eq!(pairs.len(), 2);

                // Check is_active
                assert_eq!(pairs[0].0.value, "is_active");
                match &pairs[0].1.value {
                    SpannedData::Bool(b) => assert_eq!(b.value, true),
                    _ => panic!("Expected boolean true"),
                }

                // Check is_deleted
                assert_eq!(pairs[1].0.value, "is_deleted");
                match &pairs[1].1.value {
                    SpannedData::Bool(b) => assert_eq!(b.value, false),
                    _ => panic!("Expected boolean false"),
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_parse_arrays() {
        let toml = r#"numbers = [1, 2, 3]
names = ["Alice", "Bob"]"#;
        let result = Toml.parse(toml, "test.toml");

        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse TOML");

        match parsed.value {
            SpannedData::Object(pairs) => {
                assert_eq!(pairs.len(), 2);

                // Check numbers array
                assert_eq!(pairs[0].0.value, "numbers");
                match &pairs[0].1.value {
                    SpannedData::Array(arr) => {
                        assert_eq!(arr.len(), 3);
                        for (i, item) in arr.iter().enumerate() {
                            match &item.value {
                                SpannedData::Number(n) => assert_eq!(n.value, (i + 1) as f64),
                                _ => panic!("Expected number values in array"),
                            }
                        }
                    }
                    _ => panic!("Expected array for numbers"),
                }

                // Check names array
                assert_eq!(pairs[1].0.value, "names");
                match &pairs[1].1.value {
                    SpannedData::Array(arr) => {
                        assert_eq!(arr.len(), 2);
                        match &arr[0].value {
                            SpannedData::String(s) => assert_eq!(s.value, "Alice"),
                            _ => panic!("Expected string value"),
                        }
                        match &arr[1].value {
                            SpannedData::String(s) => assert_eq!(s.value, "Bob"),
                            _ => panic!("Expected string value"),
                        }
                    }
                    _ => panic!("Expected array for names"),
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_parse_inline_tables() {
        let toml = r#"point = { x = 1, y = 2 }"#;
        let result = Toml.parse(toml, "test.toml");

        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse TOML");

        match parsed.value {
            SpannedData::Object(pairs) => {
                assert_eq!(pairs.len(), 1);
                assert_eq!(pairs[0].0.value, "point");

                match &pairs[0].1.value {
                    SpannedData::Object(inner_pairs) => {
                        assert_eq!(inner_pairs.len(), 2);

                        // Check x field
                        assert_eq!(inner_pairs[0].0.value, "x");
                        match &inner_pairs[0].1.value {
                            SpannedData::Number(n) => assert_eq!(n.value, 1.0),
                            _ => panic!("Expected number value for x"),
                        }

                        // Check y field
                        assert_eq!(inner_pairs[1].0.value, "y");
                        match &inner_pairs[1].1.value {
                            SpannedData::Number(n) => assert_eq!(n.value, 2.0),
                            _ => panic!("Expected number value for y"),
                        }
                    }
                    _ => panic!("Expected inline table"),
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_parse_tables() {
        let toml = r#"[person]
name = "Alice"
age = 25"#;
        let result = Toml.parse(toml, "test.toml");

        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse TOML");

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
                    _ => panic!("Expected table object"),
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_parse_array_of_tables() {
        let toml = r#"[[products]]
name = "Hammer"
sku = 738594937

[[products]]
name = "Nail"
sku = 284758393"#;
        let result = Toml.parse(toml, "test.toml");

        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse TOML");

        match parsed.value {
            SpannedData::Object(pairs) => {
                assert_eq!(pairs.len(), 1);
                assert_eq!(pairs[0].0.value, "products");

                match &pairs[0].1.value {
                    SpannedData::Array(arr) => {
                        assert_eq!(arr.len(), 2);

                        // Check first product
                        match &arr[0].value {
                            SpannedData::Object(product_pairs) => {
                                assert_eq!(product_pairs.len(), 2);

                                // Check name
                                assert_eq!(product_pairs[0].0.value, "name");
                                match &product_pairs[0].1.value {
                                    SpannedData::String(s) => assert_eq!(s.value, "Hammer"),
                                    _ => panic!("Expected string value for name"),
                                }

                                // Check sku
                                assert_eq!(product_pairs[1].0.value, "sku");
                                match &product_pairs[1].1.value {
                                    SpannedData::Number(n) => assert_eq!(n.value, 738594937.0),
                                    _ => panic!("Expected number value for sku"),
                                }
                            }
                            _ => panic!("Expected object for product"),
                        }

                        // Check second product
                        match &arr[1].value {
                            SpannedData::Object(product_pairs) => {
                                assert_eq!(product_pairs.len(), 2);

                                // Check name
                                assert_eq!(product_pairs[0].0.value, "name");
                                match &product_pairs[0].1.value {
                                    SpannedData::String(s) => assert_eq!(s.value, "Nail"),
                                    _ => panic!("Expected string value for name"),
                                }

                                // Check sku
                                assert_eq!(product_pairs[1].0.value, "sku");
                                match &product_pairs[1].1.value {
                                    SpannedData::Number(n) => assert_eq!(n.value, 284758393.0),
                                    _ => panic!("Expected number value for sku"),
                                }
                            }
                            _ => panic!("Expected object for product"),
                        }
                    }
                    _ => panic!("Expected array for products"),
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_parse_empty_document() {
        let toml = r#""#;
        let result = Toml.parse(toml, "test.toml");

        assert!(result.is_ok());
        let parsed = result.expect("Failed to parse TOML");

        match parsed.value {
            SpannedData::Object(pairs) => {
                assert_eq!(pairs.len(), 0);
            }
            _ => panic!("Expected empty object"),
        }
    }

    #[test]
    fn test_parse_invalid_toml() {
        let toml = r#"name = "John
age = 30"#; // Unclosed string
        let result = Toml.parse(toml, "test.toml");

        // This should fail
        assert!(result.is_err());
    }
}

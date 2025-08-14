use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct JsonSchema {
    #[serde(rename = "type")]
    type_field: Option<JsonSchemaType>,
    #[serde(default)]
    properties: HashMap<String, Box<JsonSchema>>,
    #[serde(default)]
    required: Vec<String>,
    items: Option<Box<JsonSchema>>,
    additional_properties: Option<Box<JsonSchema>>,
    description: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum JsonSchemaType {
    Single(String),
    Multiple(Vec<String>),
}

pub fn convert(json_schema_text: &str) -> String {
    let json_schema: JsonSchema =
        serde_json::from_str(json_schema_text).expect("Invalid JSON Schema");
    json_schema_to_deval(&json_schema)
}

fn convert_json_type(type_str: &str) -> String {
    match type_str {
        "string" | "number" | "integer" | "null" => type_str.to_string(),
        "boolean" => "bool".to_string(),
        "array" => "any[]".to_string(),
        "object" => "{ .. }".to_string(),
        _ => "any".to_string(),
    }
}

fn json_schema_to_deval(schema: &JsonSchema) -> String {
    // Check if it's a type specification
    if let Some(type_field) = &schema.type_field {
        match type_field {
            JsonSchemaType::Single(type_str) => {
                match type_str.as_str() {
                    "array" => {
                        if let Some(items) = &schema.items {
                            format!("{}[]", json_schema_to_deval(items))
                        } else {
                            "any[]".to_string()
                        }
                    }
                    "object" => {
                        let mut fields = Vec::new();

                        // Get required fields
                        let required: HashSet<&String> = schema.required.iter().collect();

                        for (key, prop_schema) in &schema.properties {
                            // Only include required fields since deval schema doesn't support optional fields
                            if required.contains(key) {
                                let field_type = json_schema_to_deval(prop_schema);

                                // Add documentation if available
                                let doc_comment = if let Some(desc) = &prop_schema.description {
                                    format!("/// {}\n    ", desc)
                                } else {
                                    String::new()
                                };

                                fields.push(format!("{}{}: {}", doc_comment, key, field_type));
                            }
                        }

                        format!("{{\n    {}\n}}", fields.join(",\n    "))
                    }
                    _ => convert_json_type(type_str),
                }
            }
            JsonSchemaType::Multiple(type_array) => {
                // Handle multiple types using the new | syntax
                let converted_types: Vec<String> = type_array
                    .iter()
                    .map(|type_str| {
                        if type_str == "array" {
                            if let Some(items) = &schema.items {
                                format!("{}[]", json_schema_to_deval(items))
                            } else {
                                "any[]".to_string()
                            }
                        } else {
                            convert_json_type(type_str)
                        }
                    })
                    .collect();

                if converted_types.len() == 1 {
                    converted_types[0].clone()
                } else {
                    converted_types.join(" | ")
                }
            }
        }
    } else if !schema.properties.is_empty() {
        // Object without explicit type
        let mut fields = Vec::new();

        // Get required fields
        let required: HashSet<&String> = schema.required.iter().collect();

        for (key, prop_schema) in &schema.properties {
            // Only include required fields since deval schema doesn't support optional fields
            if required.contains(key) {
                let field_type = json_schema_to_deval(prop_schema);

                // Add documentation if available
                let doc_comment = if let Some(desc) = &prop_schema.description {
                    format!("/// {}\n    ", desc)
                } else {
                    String::new()
                };

                fields.push(format!("{}{}: {}", doc_comment, key, field_type));
            }
        }

        format!("{{\n    {}\n}}", fields.join(",\n    "))
    } else if schema.additional_properties.is_some() {
        // For objects with additional properties, use a generic object for now
        "{\n}".to_string()
    } else {
        "any".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_types() {
        let json_schema = r#"{"type": ["string", "integer"]}"#;
        let result = convert(json_schema);
        assert_eq!(result, "string | integer");
    }

    #[test]
    fn test_complex_union_types() {
        let json_schema = r#"{"type": ["string", "number", "boolean", "null"]}"#;
        let result = convert(json_schema);
        assert_eq!(result, "string | number | bool | null");
    }

    #[test]
    fn test_single_type_in_array() {
        let json_schema = r#"{"type": ["string"]}"#;
        let result = convert(json_schema);
        assert_eq!(result, "string");
    }

    #[test]
    fn test_single_type() {
        let json_schema = r#"{"type": "string"}"#;
        let result = convert(json_schema);
        assert_eq!(result, "string");
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use deval_schema::compile;

    #[test]
    fn test_union_schema_compilation() {
        // Test that a schema with union types can be converted and compiled
        let json_schema = r#"{"type": ["string", "number"]}"#;
        let deval_schema = convert(json_schema);

        // Verify the conversion uses the | syntax
        assert_eq!(deval_schema, "string | number");

        // Verify that the resulting schema can be compiled
        let result = compile(&deval_schema);
        assert!(result.is_ok());
    }

    #[test]
    fn test_complex_union_schema_compilation() {
        // Test that a complex schema with union types can be converted and compiled
        let json_schema = r#"{"type": ["string", "number", "boolean", "null"]}"#;
        let deval_schema = convert(json_schema);

        // Verify the conversion uses the | syntax
        assert_eq!(deval_schema, "string | number | bool | null");

        // Verify that the resulting schema can be compiled
        let result = compile(&deval_schema);
        assert!(result.is_ok());
    }

    #[test]
    fn test_single_type_in_array_compilation() {
        // Test that a schema with a single type in an array can be converted and compiled
        let json_schema = r#"{"type": ["string"]}"#;
        let deval_schema = convert(json_schema);

        // Verify the conversion simplifies single-element arrays
        assert_eq!(deval_schema, "string");

        // Verify that the resulting schema can be compiled
        let result = compile(&deval_schema);
        assert!(result.is_ok());
    }

    #[test]
    fn test_single_type_compilation() {
        // Test that a schema with a single type can be converted and compiled
        let json_schema = r#"{"type": "string"}"#;
        let deval_schema = convert(json_schema);

        // Verify the conversion works for single types
        assert_eq!(deval_schema, "string");

        // Verify that the resulting schema can be compiled
        let result = compile(&deval_schema);
        assert!(result.is_ok());
    }
}

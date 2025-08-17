use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonSchema {
    #[serde(rename = "type")]
    type_field: Option<JsonSchemaType>,
    #[serde(default)]
    properties: HashMap<String, Box<JsonSchema>>,
    #[serde(default)]
    required: Vec<String>,
    items: Option<Box<JsonSchema>>,
    min_items: Option<i32>,
    max_items: Option<i32>,
    additional_properties: Option<AdditionalProperties>,
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum AdditionalProperties {
    Boolean(bool),
    Schema(Box<JsonSchema>),
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

fn convert_object_properties(schema: &JsonSchema) -> String {
    let mut fields = Vec::new();

    // Get required fields
    let required: HashSet<&String> = schema.required.iter().collect();

    for (key, prop_schema) in &schema.properties {
        let field_type = json_schema_to_deval(prop_schema);
        
        // Determine if the field is optional (not in required list)
        let is_optional = !required.contains(key);
        let field_name = if is_optional {
            format!("{}?", key)
        } else {
            key.clone()
        };

        // Add documentation if available
        let doc_comment = if let Some(desc) = &prop_schema.description {
            format!("/// {}\n    ", desc)
        } else {
            String::new()
        };

        fields.push(format!("{}{}: {}", doc_comment, field_name, field_type));
    }
    
    // Check if the object allows additional properties
    let allows_additional = match &schema.additional_properties {
        Some(additional) => {
            match additional {
                // If additionalProperties is explicitly false, no additional properties allowed
                AdditionalProperties::Boolean(false) => false,
                // If additionalProperties is true or a schema, additional properties are allowed
                AdditionalProperties::Boolean(true) | AdditionalProperties::Schema(_) => true,
            }
        },
        // If additionalProperties is not specified, it defaults to true
        None => true,
    };

    // Add .. if the object allows additional properties
    if allows_additional {
        fields.push("..".to_string());
    }

    if fields.is_empty() {
        if allows_additional {
            "{\n    ..\n}".to_string()
        } else {
            "{\n}".to_string()
        }
    } else {
        format!("{{\n    {}\n}}", fields.join(",\n    "))
    }
}

fn json_schema_to_deval(schema: &JsonSchema) -> String {
    // Check if it's a type specification
    if let Some(type_field) = &schema.type_field {
        match type_field {
            JsonSchemaType::Single(type_str) => match type_str.as_str() {
                "array" => {
                    let len_range = match (schema.min_items, schema.max_items) {
                        (None, None) => format!("[]"),
                        (None, Some(r)) => format!("[..={r}]"),
                        (Some(l), None) => format!("[{l}..]"),
                        (Some(l), Some(r)) => format!("[{l}..={r}]"),
                    };
                    if let Some(items) = &schema.items {
                        format!("{}{len_range}", json_schema_to_deval(items))
                    } else {
                        format!("any{len_range}")
                    }
                }
                "object" => convert_object_properties(schema),
                _ => convert_json_type(type_str),
            },
            JsonSchemaType::Multiple(type_array) => {
                // Handle multiple types using the new | syntax
                let converted_types: Vec<String> = type_array
                    .iter()
                    .map(|type_str| match type_str.as_str() {
                        "array" => {
                            if let Some(items) = &schema.items {
                                format!("{}[]", json_schema_to_deval(items))
                            } else {
                                "any[]".to_string()
                            }
                        }
                        "object" => convert_object_properties(schema),
                        _ => convert_json_type(type_str),
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
        convert_object_properties(schema)
    } else if schema.additional_properties.is_some() {
        // For objects with additional properties but no defined properties
        convert_object_properties(schema)
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

    #[test]
    fn test_object_with_properties() {
        let json_schema = r#"{
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"}
            },
            "required": ["name", "age"]
        }"#;
        let result = convert(json_schema);
        // Check that both properties are present, regardless of order
        assert!(result.contains("name: string"));
        assert!(result.contains("age: integer"));
        assert!(result.contains("{"));
        assert!(result.contains("}"));
    }

    #[test]
    fn test_object_without_type() {
        let json_schema = r#"{
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"}
            },
            "required": ["name", "age"]
        }"#;
        let result = convert(json_schema);
        // Check that all expected elements are present
        assert!(result.contains("name: string"));
        assert!(result.contains("age: integer"));
        assert!(result.contains("{"));
        assert!(result.contains("}"));
        assert!(result.contains(".."));
    }

    #[test]
    fn test_nested_object() {
        let json_schema = r#"{
            "type": "object",
            "properties": {
                "user": {
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"},
                        "age": {"type": "integer"}
                    },
                    "required": ["name", "age"]
                }
            },
            "required": ["user"]
        }"#;
        let result = convert(json_schema);
        // Check that all expected elements are present
        assert!(result.contains("user: {"));
        assert!(result.contains("name: string"));
        assert!(result.contains("age: integer"));
        assert!(result.contains("..")); // Inner object has ..
        assert!(result.contains("}")); // Closing braces
        // Should have two .. entries (one for inner object, one for outer object)
        let dotdot_count = result.matches("..").count();
        assert_eq!(dotdot_count, 2);
    }

    #[test]
    fn test_object_with_optional_properties() {
        let json_schema = r#"{
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"},
                "email": {"type": "string"}
            },
            "required": ["name"]
        }"#;
        let result = convert(json_schema);
        // Required and optional properties should be included with optional ones marked with ?
        assert!(result.contains("name: string"));
        assert!(result.contains("age?: integer"));
        assert!(result.contains("email?: string"));
    }

    #[test]
    fn test_object_with_documentation() {
        let json_schema = r#"{
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "The user's name"
                },
                "age": {
                    "type": "integer",
                    "description": "The user's age"
                }
            },
            "required": ["name", "age"]
        }"#;
        let result = convert(json_schema);
        // Check that documentation is included
        assert!(result.contains("/// The user's name"));
        assert!(result.contains("/// The user's age"));
    }

    #[test]
    fn test_object_with_no_additional_properties() {
        let json_schema = r#"{
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"}
            },
            "required": ["name"],
            "additionalProperties": false
        }"#;
        let result = convert(json_schema);
        // Should not contain .. at the end since additionalProperties is false
        assert!(!result.contains(".."));
        assert!(result.contains("name: string"));
        assert!(result.contains("age?: integer"));
        // Should end with the closing brace
        assert!(result.trim_end().ends_with("}"));
    }

    #[test]
    fn test_object_with_additional_properties_true() {
        let json_schema = r#"{
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"}
            },
            "required": ["name"],
            "additionalProperties": true
        }"#;
        let result = convert(json_schema);
        // Should contain .. at the end
        assert!(result.contains(".."));
        assert!(result.contains("name: string"));
        assert!(result.contains("age?: integer"));
    }

    #[test]
    fn test_object_with_additional_properties_schema() {
        let json_schema = r#"{
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"}
            },
            "required": ["name"],
            "additionalProperties": {
                "type": "string"
            }
        }"#;
        let result = convert(json_schema);
        // Should contain .. at the end since additionalProperties is a schema
        assert!(result.contains(".."));
        assert!(result.contains("name: string"));
        assert!(result.contains("age?: integer"));
    }

    #[test]
    fn test_object_with_default_additional_properties() {
        let json_schema = r#"{
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"}
            },
            "required": ["name"]
        }"#;
        let result = convert(json_schema);
        // Should contain .. at the end since additionalProperties defaults to true
        assert!(result.contains(".."));
        assert!(result.contains("name: string"));
        assert!(result.contains("age?: integer"));
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
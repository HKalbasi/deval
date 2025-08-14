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

fn json_schema_to_deval(schema: &JsonSchema) -> String {
    // Check if it's a type specification
    if let Some(type_field) = &schema.type_field {
        match type_field {
            JsonSchemaType::Single(type_str) => match type_str.as_str() {
                "string" | "number" | "integer" | "null" => type_str.to_string(),
                "boolean" => "bool".to_string(),
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
                _ => "any".to_string(),
            },
            JsonSchemaType::Multiple(type_array) => {
                // Handle multiple types - use the first one or "any" if complex
                if type_array.len() == 1 {
                    match type_array[0].as_str() {
                        "string" => "string".to_string(),
                        "number" | "integer" => "number".to_string(),
                        "boolean" => "bool".to_string(),
                        "array" => "any[]".to_string(),
                        _ => "any".to_string(),
                    }
                } else if type_array.iter().any(|t| t == "array") {
                    // If one of the types is array, return array of any
                    "any[]".to_string()
                } else {
                    // For multiple types, use "any" for now
                    "any".to_string()
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

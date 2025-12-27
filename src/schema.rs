use crate::value::Value;
use std::fmt;

/// Errors that can occur during schema validation
#[derive(Debug, Clone)]
pub enum ValidationError {
    TypeMismatch {
        path: String,
        expected: String,
        actual: String,
    },
    MissingField {
        path: String,
        field: String,
    },
    UnknownField {
        path: String,
        field: String,
    },
    InvalidSchema {
        path: String,
        message: String,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::TypeMismatch {
                path,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Type mismatch at '{}': expected {}, found {}",
                    path, expected, actual
                )
            }
            ValidationError::MissingField { path, field } => {
                write!(f, "Missing required field '{}' at '{}'", field, path)
            }
            ValidationError::UnknownField { path, field } => {
                write!(f, "Unknown field '{}' at '{}'", field, path)
            }
            ValidationError::InvalidSchema { path, message } => {
                write!(f, "Invalid schema definition at '{}': {}", path, message)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validate a COSY value against a schema definition.
///
/// The schema is itself a COSY Value (typically an object) where:
/// - Strings represent types: "string", "integer", "float", "boolean", "null", "any"
/// - Arrays represent lists of a single type: ["string"] (array of strings)
/// - Objects represent structural schemas: { name: "string", age: "integer" }
///
/// Optional fields can be denoted by appending '?' to the key name in the schema object,
/// BUT since COSY keys are strings, we might need a convention.
///
/// For v1, let's stick to strict required fields.
pub fn validate(instance: &Value, schema: &Value) -> Result<(), ValidationError> {
    validate_recursive(instance, schema, "$")
}

fn validate_recursive(instance: &Value, schema: &Value, path: &str) -> Result<(), ValidationError> {
    match schema {
        // Schema is a type name string (e.g., "string", "integer")
        Value::String(type_name) => validate_type(instance, type_name, path),

        // Schema is an object definition
        Value::Object(schema_obj) => {
            if let Value::Object(instance_obj) = instance {
                // 1. Check that all required fields in schema exist in instance
                for (key, sub_schema) in schema_obj {
                    if !instance_obj.contains_key(key) {
                        return Err(ValidationError::MissingField {
                            path: path.to_string(),
                            field: key.clone(),
                        });
                    }
                    validate_recursive(
                        &instance_obj[key],
                        sub_schema,
                        &format!("{}.{}", path, key),
                    )?;
                }

                // 2. Check for unknown fields in instance (Strict Mode)
                for key in instance_obj.keys() {
                    if !schema_obj.contains_key(key) {
                        return Err(ValidationError::UnknownField {
                            path: path.to_string(),
                            field: key.clone(),
                        });
                    }
                }
                Ok(())
            } else {
                Err(ValidationError::TypeMismatch {
                    path: path.to_string(),
                    expected: "object".to_string(),
                    actual: instance.type_name().to_string(),
                })
            }
        }

        // Schema is an array definition (e.g., ["integer"])
        Value::Array(schema_arr) => {
            if schema_arr.len() != 1 {
                return Err(ValidationError::InvalidSchema {
                    path: path.to_string(),
                    message: "Array schema must contain exactly one element specifier".to_string(),
                });
            }

            let item_schema = &schema_arr[0];

            if let Value::Array(instance_arr) = instance {
                for (i, item) in instance_arr.iter().enumerate() {
                    validate_recursive(item, item_schema, &format!("{}[{}]", path, i))?;
                }
                Ok(())
            } else {
                Err(ValidationError::TypeMismatch {
                    path: path.to_string(),
                    expected: "array".to_string(),
                    actual: instance.type_name().to_string(),
                })
            }
        }

        _ => Err(ValidationError::InvalidSchema {
            path: path.to_string(),
            message: format!("Unsupported schema value type: {}", schema.type_name()),
        }),
    }
}

fn validate_type(instance: &Value, type_name: &str, path: &str) -> Result<(), ValidationError> {
    let actual_type = instance.type_name();

    let is_valid = match type_name {
        "any" => true,
        "string" => matches!(instance, Value::String(_)),
        "integer" => matches!(instance, Value::Integer(_)),
        "float" => matches!(instance, Value::Float(_)),
        "boolean" | "bool" => matches!(instance, Value::Bool(_)),
        "null" => matches!(instance, Value::Null),
        "number" => matches!(instance, Value::Integer(_) | Value::Float(_)),
        _ => {
            return Err(ValidationError::InvalidSchema {
                path: path.to_string(),
                message: format!("Unknown type '{}'", type_name),
            });
        }
    };

    if is_valid {
        Ok(())
    } else {
        Err(ValidationError::TypeMismatch {
            path: path.to_string(),
            expected: type_name.to_string(),
            actual: actual_type.to_string(),
        })
    }
}

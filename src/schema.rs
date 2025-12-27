use crate::value::Value;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationLevel {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub struct ValidationItem {
    pub level: ValidationLevel,
    pub path: String,
    pub message: String,
}

impl fmt::Display for ValidationItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let level_str = match self.level {
            ValidationLevel::Error => "Error",
            ValidationLevel::Warning => "Warning",
        };
        write!(f, "[{} at {}] {}", level_str, self.path, self.message)
    }
}

pub type ValidationReport = Vec<ValidationItem>;

/// Validate a COSY value against a schema definition.
pub fn validate(instance: &Value, schema: &Value) -> Result<ValidationReport, ValidationItem> {
    let mut report = Vec::new();
    validate_recursive(instance, schema, "$", &mut report)?;
    Ok(report)
}

fn validate_recursive(
    instance: &Value,
    schema: &Value,
    path: &str,
    report: &mut ValidationReport,
) -> Result<(), ValidationItem> {
    // 1. Resolve Extended Schema Syntax: { type: "string", deprecated: "msg" }
    // If schema is an object with "type" key (and it's a string), treat it as extended definition.
    let (effective_type_schema, deprecation) = if let Value::Object(schema_obj) = schema {
        if let Some(Value::String(_)) = schema_obj.get("type") {
            // It is an extended schema definition
            let type_def = schema_obj.get("type").unwrap(); // We checked it exists
            let deprecated_msg = if let Some(Value::String(msg)) = schema_obj.get("deprecated") {
                Some(msg.clone())
            } else {
                None
            };
            (type_def, deprecated_msg)
        } else {
            // Just a structural object schema
            (schema, None)
        }
    } else {
        (schema, None)
    };

    // 2. Report Deprecation Warning if applicable
    if let Some(msg) = deprecation {
        report.push(ValidationItem {
            level: ValidationLevel::Warning,
            path: path.to_string(),
            message: format!("Deprecated usage: {}", msg),
        });
    }

    // 3. Validate Type / Structure
    match effective_type_schema {
        Value::String(type_name) => validate_type(instance, type_name, path, report),

        Value::Object(schema_obj) => {
            if let Value::Object(instance_obj) = instance {
                // Check required fields
                for (key, sub_schema) in schema_obj {
                    if !instance_obj.contains_key(key) {
                        report.push(ValidationItem {
                            level: ValidationLevel::Error,
                            path: path.to_string(),
                            message: format!("Missing required field '{}'", key),
                        });
                    } else {
                        validate_recursive(
                            &instance_obj[key],
                            sub_schema,
                            &format!("{}.{}", path, key),
                            report,
                        )?;
                    }
                }

                // Check unknown fields and typos
                let schema_keys: Vec<String> = schema_obj.keys().cloned().collect();
                for key in instance_obj.keys() {
                    if !schema_obj.contains_key(key) {
                        let mut msg = format!("Unknown field '{}'", key);

                        // Typo Suggestion
                        if let Some(best_match) =
                            crate::suggest::find_best_match(key, &schema_keys, 2)
                        {
                            msg.push_str(&format!("; did you mean '{}'?", best_match));
                        }

                        report.push(ValidationItem {
                            level: ValidationLevel::Error,
                            path: path.to_string(),
                            message: msg,
                        });
                    }
                }
                Ok(())
            } else {
                report.push(ValidationItem {
                    level: ValidationLevel::Error,
                    path: path.to_string(),
                    message: format!("Expected object, found {}", instance.type_name()),
                });
                Ok(())
            }
        }

        Value::Array(schema_arr) => {
            if schema_arr.len() != 1 {
                return Err(ValidationItem {
                    level: ValidationLevel::Error,
                    path: path.to_string(),
                    message: "Array schema must contain exactly one element specifier".to_string(),
                });
            }

            let item_schema = &schema_arr[0];

            if let Value::Array(instance_arr) = instance {
                for (i, item) in instance_arr.iter().enumerate() {
                    validate_recursive(item, item_schema, &format!("{}[{}]", path, i), report)?;
                }
                Ok(())
            } else {
                report.push(ValidationItem {
                    level: ValidationLevel::Error,
                    path: path.to_string(),
                    message: format!("Expected array, found {}", instance.type_name()),
                });
                Ok(())
            }
        }

        _ => Err(ValidationItem {
            level: ValidationLevel::Error,
            path: path.to_string(),
            message: format!(
                "Unsupported schema value type: {}",
                effective_type_schema.type_name()
            ),
        }),
    }
}

fn validate_type(
    instance: &Value,
    type_name: &str,
    path: &str,
    report: &mut ValidationReport,
) -> Result<(), ValidationItem> {
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
            return Err(ValidationItem {
                level: ValidationLevel::Error,
                path: path.to_string(),
                message: format!("Unknown type '{}'", type_name),
            });
        }
    };

    if !is_valid {
        report.push(ValidationItem {
            level: ValidationLevel::Error,
            path: path.to_string(),
            message: format!(
                "Type mismatch: expected {}, found {}",
                type_name, actual_type
            ),
        });
    }
    Ok(())
}

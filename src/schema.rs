use crate::value::{Value, ValueKind};
use std::fmt;

pub mod suggest;

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
    // 1. Resolve Extended Schema Syntax: { type: "string", deprecated: "msg", optional: true }
    let (effective_type_schema, deprecation, _) = extract_metadata(schema);

    // 2. Report Deprecation Warning if applicable
    if let Some(msg) = deprecation {
        report.push(ValidationItem {
            level: ValidationLevel::Warning,
            path: path.to_string(),
            message: format!("Deprecated usage: {}", msg),
        });
    }

    // 3. Validate Type / Structure
    match &effective_type_schema.kind {
        ValueKind::String(type_name) => validate_type(instance, type_name, path, report),

        ValueKind::Object(schema_obj) => {
            if let ValueKind::Object(instance_obj) = &instance.kind {
                // Check required fields
                for (key, sub_schema) in schema_obj {
                    if !instance_obj.contains_key(key) {
                        let (_, _, is_optional) = extract_metadata(sub_schema);
                        if !is_optional {
                            report.push(ValidationItem {
                                level: ValidationLevel::Error,
                                path: path.to_string(),
                                message: format!("Missing required field '{}'", key),
                            });
                        }
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
                        if let Some(best_match) = suggest::find_best_match(key, &schema_keys, 2) {
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

        ValueKind::Array(schema_arr) => {
            if schema_arr.len() != 1 {
                return Err(ValidationItem {
                    level: ValidationLevel::Error,
                    path: path.to_string(),
                    message: "Array schema must contain exactly one element specifier".to_string(),
                });
            }

            let item_schema = &schema_arr[0];

            if let ValueKind::Array(instance_arr) = &instance.kind {
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
        "string" => matches!(instance.kind, ValueKind::String(_)),
        "integer" => matches!(instance.kind, ValueKind::Integer(_)),
        "float" => matches!(instance.kind, ValueKind::Float(_)),
        "boolean" | "bool" => matches!(instance.kind, ValueKind::Bool(_)),
        "null" => matches!(instance.kind, ValueKind::Null),
        "number" => matches!(instance.kind, ValueKind::Integer(_) | ValueKind::Float(_)),
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

fn extract_metadata(schema: &Value) -> (&Value, Option<String>, bool) {
    if let ValueKind::Object(schema_obj) = &schema.kind {
        if let Some(type_val) = schema_obj.get("type") {
            if let ValueKind::String(_) = type_val.kind {
                // Extended schema definition
                let type_def = schema_obj.get("type").unwrap();

                let deprecated_msg = if let Some(dep_val) = schema_obj.get("deprecated") {
                    if let ValueKind::String(msg) = &dep_val.kind {
                        Some(msg.clone())
                    } else {
                        None
                    }
                } else {
                    None
                };

                let optional = if let Some(opt_val) = schema_obj.get("optional") {
                    if let ValueKind::Bool(b) = &opt_val.kind {
                        *b
                    } else {
                        false
                    }
                } else {
                    false
                };

                return (type_def, deprecated_msg, optional);
            }
        }
    }
    (schema, None, false)
}

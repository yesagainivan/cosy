use cosy::schema::{ValidationItem, ValidationLevel};
use cosy::{Value, from_str, schema};

#[test]
fn test_validate_basic_types() {
    let schema_str = r#"{ name: "string", age: "integer" }"#;
    let valid_str = r#"{ name: "Alice", age: 30 }"#;
    let invalid_str = r#"{ name: "Alice", age: "30" }"#;

    let schema: Value = from_str(schema_str).unwrap();
    let valid: Value = from_str(valid_str).unwrap();
    let invalid: Value = from_str(invalid_str).unwrap();

    let report = schema::validate(&valid, &schema).unwrap();
    assert!(report.is_empty(), "Expected no validation errors");

    let report_invalid = schema::validate(&invalid, &schema).unwrap();
    assert_eq!(report_invalid.len(), 1);
    assert_eq!(report_invalid[0].level, ValidationLevel::Error);
    assert!(report_invalid[0].message.contains("Type mismatch"));
}

#[test]
fn test_validate_nested_object() {
    let schema_str = r#"{
        server: {
            host: "string"
            port: "integer"
        }
    }"#;

    let valid_str = r#"{
        server: {
            host: "localhost"
            port: 8080
        }
    }"#;

    let schema: Value = from_str(schema_str).unwrap();
    let valid: Value = from_str(valid_str).unwrap();

    let report = schema::validate(&valid, &schema).unwrap();
    assert!(report.is_empty());
}

#[test]
fn test_validate_missing_field() {
    let schema_str = r#"{ required: "string" }"#;
    let invalid_str = r#"{ other: "string" }"#;

    let schema: Value = from_str(schema_str).unwrap();
    let invalid: Value = from_str(invalid_str).unwrap();

    let report = schema::validate(&invalid, &schema).unwrap();
    // Should have missing field AND unknown field
    assert!(
        report
            .iter()
            .any(|i| i.message.contains("Missing required field"))
    );
    assert!(report.iter().any(|i| i.message.contains("Unknown field")));
}

#[test]
fn test_validate_unknown_field_strict() {
    let schema_str = r#"{ allow: "string" }"#;
    let invalid_str = r#"{ allow: "ok", extra: "no" }"#;

    let schema: Value = from_str(schema_str).unwrap();
    let invalid: Value = from_str(invalid_str).unwrap();

    let report = schema::validate(&invalid, &schema).unwrap();
    assert_eq!(report.len(), 1);
    assert!(report[0].message.contains("Unknown field 'extra'"));
}

#[test]
fn test_validate_nested_array_of_objects() {
    let schema_str = r#"{
        users: [{ name: "string" }]
    }"#;

    let valid_str = r#"{
        users: [
            { name: "Alice" }
            { name: "Bob" }
        ]
    }"#;

    let schema: Value = from_str(schema_str).unwrap();
    let valid: Value = from_str(valid_str).unwrap();

    let report = schema::validate(&valid, &schema).unwrap();
    assert!(report.is_empty());
}

#[test]
fn test_validate_array_mismatch() {
    let schema_str = r#"{ list: ["integer"] }"#;
    let schema: Value = from_str(schema_str).unwrap();

    let invalid = from_str(r#"{ list: [1, "bad", 3] }"#).unwrap();
    let report = schema::validate(&invalid, &schema).unwrap();

    assert_eq!(report.len(), 1);
    assert!(report[0].message.contains("Type mismatch"));
    assert_eq!(report[0].path, "$.list[1]");
}

// NEW TESTS for Strict Mode
#[test]
fn test_typo_suggestion() {
    let schema_str = r#"{ port: "integer" }"#;
    let schema: Value = from_str(schema_str).unwrap();

    let invalid = from_str(r#"{ prt: 8080 }"#).unwrap(); // Typo: prt vs port

    let report = schema::validate(&invalid, &schema).unwrap();
    // Expect: Unknown field 'prt'; did you mean 'port'? AND Missing required field 'port'

    let unknown_err = report
        .iter()
        .find(|i| i.message.contains("Unknown field"))
        .unwrap();
    assert!(unknown_err.message.contains("did you mean 'port'?"));
}

#[test]
fn test_deprecation_warning() {
    let schema_str = r#"{
        host: "string"
        old_port: { type: "integer", deprecated: "Use 'port' instead" }
    }"#;
    let schema: Value = from_str(schema_str).unwrap();

    let valid_but_deprecated = from_str(
        r#"{
        host: "localhost"
        old_port: 8080
    }"#,
    )
    .unwrap();

    let report = schema::validate(&valid_but_deprecated, &schema).unwrap();

    // Should have 1 warning
    assert_eq!(report.len(), 1);
    assert_eq!(report[0].level, ValidationLevel::Warning);
    assert!(
        report[0]
            .message
            .contains("Deprecated usage: Use 'port' instead")
    );
}

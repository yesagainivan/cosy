use cosy::schema::ValidationError;
use cosy::{from_str, schema};

#[test]
fn test_validate_basic_types() {
    let schema_str = r#"{
        s: "string"
        i: "integer"
        f: "float"
        b: "boolean"
        n: "null"
        any: "any"
    }"#;
    let schema = from_str(schema_str).unwrap();

    let valid_str = r#"{
        s: "hello"
        i: 42
        f: 3.14
        b: true
        n: null
        any: [1, 2] // Any can be anything
    }"#;
    let valid = from_str(valid_str).unwrap();

    assert!(schema::validate(&valid, &schema).is_ok());

    let invalid_str = r#"{
        s: 42 // Expect string
        i: 42
        f: 3.14
        b: true
        n: null
        any: "ok"
    }"#;
    let invalid = from_str(invalid_str).unwrap();

    let err = schema::validate(&invalid, &schema).unwrap_err();
    match err {
        ValidationError::TypeMismatch {
            path,
            expected,
            actual,
        } => {
            assert_eq!(path, "$.s");
            assert_eq!(expected, "string");
            assert_eq!(actual, "integer");
        }
        _ => panic!("Expected TypeMismatch, got {:?}", err),
    }
}

#[test]
fn test_validate_nested_object() {
    let schema_str = r#"{
        server: {
            host: "string"
            port: "integer"
        }
    }"#;
    let schema = from_str(schema_str).unwrap();

    let valid_str = r#"{
        server: {
            host: "localhost"
            port: 8080
        }
    }"#;
    let valid = from_str(valid_str).unwrap();
    assert!(schema::validate(&valid, &schema).is_ok());

    let invalid_str = r#"{
        server: {
            host: "localhost"
            port: "8080" // Wrong type
        }
    }"#;
    let invalid = from_str(invalid_str).unwrap();

    let err = schema::validate(&invalid, &schema).unwrap_err();
    match err {
        ValidationError::TypeMismatch {
            path,
            expected,
            actual,
        } => {
            assert_eq!(path, "$.server.port");
            assert_eq!(expected, "integer");
            assert_eq!(actual, "string");
        }
        _ => panic!("Expected TypeMismatch, got {:?}", err),
    }
}

#[test]
fn test_validate_missing_field() {
    let schema_str = r#"{ required: "string" }"#;
    let schema = from_str(schema_str).unwrap();

    let instance = from_str("{}").unwrap();

    let err = schema::validate(&instance, &schema).unwrap_err();
    match err {
        ValidationError::MissingField { path, field } => {
            assert_eq!(path, "$");
            assert_eq!(field, "required");
        }
        _ => panic!("Expected MissingField, got {:?}", err),
    }
}

#[test]
fn test_validate_unknown_field_strict() {
    let schema_str = r#"{ allowed: "string" }"#;
    let schema = from_str(schema_str).unwrap();

    let instance = from_str(r#"{ allowed: "ok", unknown: "no" }"#).unwrap();

    let err = schema::validate(&instance, &schema).unwrap_err();
    match err {
        ValidationError::UnknownField { path, field } => {
            assert_eq!(path, "$");
            assert_eq!(field, "unknown");
        }
        _ => panic!("Expected UnknownField, got {:?}", err),
    }
}

#[test]
fn test_validate_array() {
    let schema_str = r#"{ list: ["integer"] }"#;
    let schema = from_str(schema_str).unwrap();

    let valid = from_str(r#"{ list: [1, 2, 3] }"#).unwrap();
    assert!(schema::validate(&valid, &schema).is_ok());

    let invalid = from_str(r#"{ list: [1, "bad", 3] }"#).unwrap();
    let err = schema::validate(&invalid, &schema).unwrap_err();
    match err {
        ValidationError::TypeMismatch {
            path,
            expected,
            actual,
        } => {
            assert_eq!(path, "$.list[1]");
            assert_eq!(expected, "integer");
            assert_eq!(actual, "string");
        }
        _ => panic!("Expected TypeMismatch, got {:?}", err),
    }
}

#[test]
fn test_validate_nested_array_of_objects() {
    let schema_str = r#"{
        users: [
            { id: "integer", name: "string" }
        ]
    }"#;
    let schema = from_str(schema_str).unwrap();

    let valid = from_str(
        r#"{
        users: [
            { id: 1, name: "Alice" },
            { id: 2, name: "Bob" }
        ]
    }"#,
    )
    .unwrap();
    assert!(schema::validate(&valid, &schema).is_ok());

    let invalid = from_str(
        r#"{
        users: [
            { id: 1, name: "Alice" },
            { id: 2 } // Missing name
        ]
    }"#,
    )
    .unwrap();
    let err = schema::validate(&invalid, &schema).unwrap_err();
    match err {
        ValidationError::MissingField { path, field } => {
            assert_eq!(path, "$.users[1]");
            assert_eq!(field, "name");
        }
        _ => panic!("Expected MissingField, got {:?}", err),
    }
}

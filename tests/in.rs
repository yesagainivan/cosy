// tests/integration_tests.rs
// Place this file in tests/ directory at the root of your project

use cosy::from_str;
use cosy::{CosynError, Value};
use indexmap::IndexMap;

// ============================================================================
// BASIC VALUE PARSING TESTS
// ============================================================================

#[test]
fn test_null() {
    let value = from_str("null").unwrap();
    assert_eq!(value, Value::Null);
}

#[test]
fn test_booleans() {
    assert_eq!(from_str("true").unwrap(), Value::Bool(true));
    assert_eq!(from_str("false").unwrap(), Value::Bool(false));
}

#[test]
fn test_integers() {
    assert_eq!(from_str("42").unwrap(), Value::Integer(42));
    assert_eq!(from_str("-42").unwrap(), Value::Integer(-42));
    assert_eq!(from_str("0").unwrap(), Value::Integer(0));
}

#[test]
fn test_floats() {
    assert_eq!(from_str("3.14").unwrap(), Value::Float(3.14));
    assert_eq!(from_str("-3.14").unwrap(), Value::Float(-3.14));
    assert_eq!(from_str("1e10").unwrap(), Value::Float(1e10));
    assert_eq!(from_str("1.5e-5").unwrap(), Value::Float(1.5e-5));
}

#[test]
fn test_strings() {
    assert_eq!(
        from_str(r#""hello""#).unwrap(),
        Value::String("hello".to_string())
    );
    assert_eq!(
        from_str(r#""hello world""#).unwrap(),
        Value::String("hello world".to_string())
    );
}

#[test]
fn test_string_escapes() {
    assert_eq!(
        from_str(r#""hello\nworld""#).unwrap(),
        Value::String("hello\nworld".to_string())
    );
    assert_eq!(
        from_str(r#""tab\there""#).unwrap(),
        Value::String("tab\there".to_string())
    );
    assert_eq!(
        from_str(r#""quote\"inside""#).unwrap(),
        Value::String("quote\"inside".to_string())
    );
    assert_eq!(
        from_str(r#""backslash\\here""#).unwrap(),
        Value::String("backslash\\here".to_string())
    );
}

// ============================================================================
// ARRAY PARSING TESTS
// ============================================================================

#[test]
fn test_empty_array() {
    let value = from_str("[]").unwrap();
    assert_eq!(value, Value::Array(vec![]));
}

#[test]
fn test_simple_array() {
    let value = from_str("[1, 2, 3]").unwrap();
    let expected = Value::Array(vec![
        Value::Integer(1),
        Value::Integer(2),
        Value::Integer(3),
    ]);
    assert_eq!(value, expected);
}

#[test]
fn test_mixed_array() {
    let value = from_str(r#"[1, "hello", 3.14, true, null]"#).unwrap();
    let expected = Value::Array(vec![
        Value::Integer(1),
        Value::String("hello".to_string()),
        Value::Float(3.14),
        Value::Bool(true),
        Value::Null,
    ]);
    assert_eq!(value, expected);
}

#[test]
fn test_nested_array() {
    let value = from_str("[[1, 2], [3, 4]]").unwrap();
    let expected = Value::Array(vec![
        Value::Array(vec![Value::Integer(1), Value::Integer(2)]),
        Value::Array(vec![Value::Integer(3), Value::Integer(4)]),
    ]);
    assert_eq!(value, expected);
}

#[test]
fn test_array_with_trailing_comma() {
    let value = from_str("[1, 2, 3,]").unwrap();
    let expected = Value::Array(vec![
        Value::Integer(1),
        Value::Integer(2),
        Value::Integer(3),
    ]);
    assert_eq!(value, expected);
}

#[test]
fn test_array_with_newlines() {
    let input = "[
        1,
        2,
        3
    ]";
    let value = from_str(input).unwrap();
    let expected = Value::Array(vec![
        Value::Integer(1),
        Value::Integer(2),
        Value::Integer(3),
    ]);
    assert_eq!(value, expected);
}

#[test]
fn test_array_newlines_as_separators() {
    let input = "[
        1
        2
        3
    ]";
    let value = from_str(input).unwrap();
    let expected = Value::Array(vec![
        Value::Integer(1),
        Value::Integer(2),
        Value::Integer(3),
    ]);
    assert_eq!(value, expected);
}

// ============================================================================
// OBJECT PARSING TESTS
// ============================================================================

#[test]
fn test_empty_object() {
    let value = from_str("{}").unwrap();
    assert_eq!(value, Value::Object(IndexMap::new()));
}

#[test]
fn test_simple_object() {
    let input = r#"{name: "Alice", age: 30}"#;
    let value = from_str(input).unwrap();

    if let Value::Object(obj) = value {
        assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(obj.get("age"), Some(&Value::Integer(30)));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_object_with_string_keys() {
    let input = r#"{"name": "Bob", "age": 25}"#;
    let value = from_str(input).unwrap();

    if let Value::Object(obj) = value {
        assert_eq!(obj.get("name"), Some(&Value::String("Bob".to_string())));
        assert_eq!(obj.get("age"), Some(&Value::Integer(25)));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_object_with_various_values() {
    let input = r#"{
        string: "hello",
        number: 42,
        float: 3.14,
        bool: true,
        null_val: null
    }"#;
    let value = from_str(input).unwrap();

    if let Value::Object(obj) = value {
        assert_eq!(obj.get("string"), Some(&Value::String("hello".to_string())));
        assert_eq!(obj.get("number"), Some(&Value::Integer(42)));
        assert_eq!(obj.get("float"), Some(&Value::Float(3.14)));
        assert_eq!(obj.get("bool"), Some(&Value::Bool(true)));
        assert_eq!(obj.get("null_val"), Some(&Value::Null));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_nested_object() {
    let input = r#"{
        person: {
            name: "Alice",
            age: 30
        }
    }"#;
    let value = from_str(input).unwrap();

    if let Value::Object(obj) = value {
        if let Some(Value::Object(person)) = obj.get("person") {
            assert_eq!(
                person.get("name"),
                Some(&Value::String("Alice".to_string()))
            );
            assert_eq!(person.get("age"), Some(&Value::Integer(30)));
        } else {
            panic!("Expected nested object");
        }
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_object_with_array_values() {
    let input = r#"{
        scores: [95, 87, 92],
        names: ["Alice", "Bob"]
    }"#;
    let value = from_str(input).unwrap();

    if let Value::Object(obj) = value {
        if let Some(Value::Array(scores)) = obj.get("scores") {
            assert_eq!(scores.len(), 3);
            assert_eq!(scores[0], Value::Integer(95));
        } else {
            panic!("Expected array for scores");
        }
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_object_with_newlines_as_separators() {
    let input = r#"{
        name: "Alice"
        age: 30
        active: true
    }"#;
    let value = from_str(input).unwrap();

    if let Value::Object(obj) = value {
        assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(obj.get("age"), Some(&Value::Integer(30)));
        assert_eq!(obj.get("active"), Some(&Value::Bool(true)));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_object_with_trailing_comma() {
    let input = r#"{name: "Alice", age: 30,}"#;
    let value = from_str(input).unwrap();

    if let Value::Object(obj) = value {
        assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(obj.get("age"), Some(&Value::Integer(30)));
    } else {
        panic!("Expected object");
    }
}

// ============================================================================
// COMPLEX DOCUMENT TESTS
// ============================================================================

#[test]
fn test_complex_document() {
    let input = r#"{
        users: [
            {
                id: 1,
                name: "Alice",
                email: "alice@example.com",
                tags: ["admin", "developer"]
            },
            {
                id: 2,
                name: "Bob",
                email: "bob@example.com",
                tags: ["user"]
            }
        ],
        count: 2,
        active: true
    }"#;

    let value = from_str(input).unwrap();

    if let Value::Object(root) = value {
        if let Some(Value::Array(users)) = root.get("users") {
            assert_eq!(users.len(), 2);

            if let Value::Object(alice) = &users[0] {
                assert_eq!(alice.get("name"), Some(&Value::String("Alice".to_string())));
                assert_eq!(alice.get("id"), Some(&Value::Integer(1)));
            } else {
                panic!("Expected object for first user");
            }
        } else {
            panic!("Expected array for users");
        }
    } else {
        panic!("Expected object");
    }
}

// ============================================================================
// COMMENTS AND WHITESPACE TESTS
// ============================================================================

#[test]
fn test_line_comments_in_object() {
    let input = r#"{
        // This is a comment
        name: "Alice",
        // Another comment
        age: 30
    }"#;
    let value = from_str(input).unwrap();

    if let Value::Object(obj) = value {
        assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(obj.get("age"), Some(&Value::Integer(30)));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_comments_in_array() {
    let input = r#"[
        1,
        // comment here
        2,
        3 // inline comment
    ]"#;
    let value = from_str(input).unwrap();
    let expected = Value::Array(vec![
        Value::Integer(1),
        Value::Integer(2),
        Value::Integer(3),
    ]);
    assert_eq!(value, expected);
}

// ============================================================================
// ERROR TESTS
// ============================================================================

#[test]
fn test_error_unexpected_token() {
    let result = from_str("42 99");
    assert!(result.is_err());
}

#[test]
fn test_error_unterminated_string() {
    let result = from_str(r#""unterminated"#);
    assert!(result.is_err());
}

#[test]
fn test_error_invalid_escape() {
    let result = from_str(r#""invalid\x""#);
    assert!(result.is_err());
}

#[test]
fn test_error_missing_colon_in_object() {
    let result = from_str(r#"{name "Alice"}"#);
    assert!(result.is_err());
}

#[test]
fn test_error_missing_closing_brace() {
    let result = from_str(r#"{name: "Alice""#);
    assert!(result.is_err());
}

#[test]
fn test_error_invalid_number() {
    let result = from_str("1.2.3");
    assert!(result.is_err());
}

#[test]
fn test_error_message_has_position() {
    let result = from_str(
        r#"{
        name: "Alice"
        age: "not a number" invalid
    }"#,
    );

    assert!(result.is_err());
    let err = result.unwrap_err();

    // Now we can access position information!
    assert!(err.line() > 0);
    assert!(err.column() > 0);
    assert!(!err.message().is_empty());

    // // Could also match on the specific error type
    // match err {
    //     CosynError::Parse(_) => {
    //         // Handle parse error specifically
    //     }
    //     CosynError::Lex(_) => {
    //         // Handle lex error specifically
    //     }
    // }
}

#[test]
fn test_lex_error_position() {
    let result = from_str("{ @ }");
    assert!(result.is_err());
    let err = result.unwrap_err();

    match err {
        CosynError::Lex(_) => {
            assert_eq!(err.column(), 3);
        }
        _ => panic!("Expected lex error"),
    }
}

#[test]
fn test_parse_error_position() {
    let result = from_str("[1, 2, 3 4]");
    assert!(result.is_err());
    let err = result.unwrap_err();

    match err {
        CosynError::Parse(_) => {
            // Position info should be available
            assert!(err.line() > 0);
        }
        _ => panic!("Expected parse error"),
    }
}

#[test]
fn test_string_with_newlines_deserialization() {
    let original_str = "line1\nline2";
    // The input string as it would be serialized and then parsed by `from_str`.
    // The `r#""...""#` raw string literal ensures that `\n` is interpreted by the parser as an actual newline character escape,
    // similar to how `test_string_escapes` is written.
    let input_for_parser = r#""line1\nline2""#;
    let deserialized_value = from_str(input_for_parser).unwrap();
    assert_eq!(deserialized_value, Value::String(original_str.to_string()));
}

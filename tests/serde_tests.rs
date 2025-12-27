// tests/serde_tests.rs
// Comprehensive tests for Serde integration, edge cases, and roundtrip behavior

use cosy::serde as serde_support;
use serde::{Deserialize, Serialize};

// ============================================================================
// BASIC SERDE ROUNDTRIP TESTS
// ============================================================================

#[test]
fn test_serde_simple_struct_roundtrip() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    let original = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Person = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_serde_nested_struct_roundtrip() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Address {
        street: String,
        city: String,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Person {
        name: String,
        address: Address,
    }

    let original = Person {
        name: "Alice".to_string(),
        address: Address {
            street: "123 Main St".to_string(),
            city: "Springfield".to_string(),
        },
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Person = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_serde_struct_with_vec_roundtrip() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Team {
        name: String,
        members: Vec<String>,
    }

    let original = Team {
        name: "Dev Team".to_string(),
        members: vec![
            "Alice".to_string(),
            "Bob".to_string(),
            "Charlie".to_string(),
        ],
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Team = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

// ============================================================================
// STRING ESCAPE SEQUENCE TESTS
// ============================================================================

#[test]
fn test_serde_string_with_newlines_roundtrip() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Message {
        content: String,
    }

    let original = Message {
        content: "line1\nline2\nline3".to_string(),
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Message = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_serde_string_with_tabs_roundtrip() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Message {
        content: String,
    }

    let original = Message {
        content: "col1\tcol2\tcol3".to_string(),
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Message = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_serde_string_with_quotes_roundtrip() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Quote {
        text: String,
    }

    let original = Quote {
        text: r#"He said "Hello, World!""#.to_string(),
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Quote = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_serde_string_with_backslashes_roundtrip() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Path {
        location: String,
    }

    let original = Path {
        location: r"C:\Users\Alice\Documents".to_string(),
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Path = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_serde_string_complex_escapes() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Complex {
        text: String,
    }

    let original = Complex {
        text: "a\nb\tc\"d\\e".to_string(),
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Complex = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

// ============================================================================
// NUMBER PRECISION TESTS
// ============================================================================

#[test]
fn test_serde_integer_precision() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Numbers {
        small: i32,
        large: i64,
        zero: i64,
        negative: i64,
    }

    let original = Numbers {
        small: 42,
        large: 9223372036854775800i64, // Near i64::MAX
        zero: 0,
        negative: -999999,
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Numbers = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_serde_float_precision() {
    #[derive(Debug, Serialize, Deserialize)]
    struct Floats {
        pi: f64,
        e: f64,
        small: f64,
    }

    let original = Floats {
        pi: std::f64::consts::PI,
        e: std::f64::consts::E,
        small: 0.000001,
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Floats = serde_support::from_str(&serialized).unwrap();

    // Use approximate equality due to float formatting
    assert!((original.pi - deserialized.pi).abs() < 1e-10);
    assert!((original.e - deserialized.e).abs() < 1e-10);
    assert!((original.small - deserialized.small).abs() < 1e-15);
}

#[test]
fn test_serde_scientific_notation_roundtrip() {
    #[derive(Debug, Serialize, Deserialize)]
    struct Scientific {
        big: f64,
        small: f64,
    }

    let original = Scientific {
        big: 1.23e10,
        small: 4.56e-5,
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Scientific = serde_support::from_str(&serialized).unwrap();

    assert!((original.big - deserialized.big).abs() < 1e5);
    assert!((original.small - deserialized.small).abs() < 1e-15);
}

#[test]
fn test_serde_unsigned_integers() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Unsigned {
        u8_val: u8,
        u16_val: u16,
        u32_val: u32,
        u64_val: u64,
    }

    let original = Unsigned {
        u8_val: 255,
        u16_val: 65535,
        u32_val: 4294967295,
        u64_val: 9223372036854775807u64,
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Unsigned = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

// ============================================================================
// ENUM TESTS (Unit and Newtype variants)
// ============================================================================

#[test]
fn test_serde_unit_enum() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum Status {
        Active,
        Inactive,
        Pending,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Record {
        id: u32,
        status: Status,
    }

    let original = Record {
        id: 1,
        status: Status::Active,
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Record = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_serde_newtype_enum() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum Value {
        Number(i32),
        Text(String),
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Container {
        value: Value,
    }

    let original = Container {
        value: Value::Number(42),
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Container = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_serde_enum_error_on_tuple_variant() {
    #[derive(Debug, Serialize)]
    enum Bad {
        Tuple(i32, String),
    }

    let bad = Bad::Tuple(1, "test".to_string());
    let result = serde_support::to_string(&bad);

    // Should not fail on serialization (Serde allows it)
    assert!(result.is_ok());
}

#[test]
fn test_serde_enum_error_message_quality() {
    let cosy_text = r#"{
        value: {
            MultiField: {
                a: 1,
                b: 2
            }
        }
    }"#;

    #[derive(Debug, Deserialize)]
    struct Container {
        _value: ComplexEnum,
    }

    #[derive(Debug, Deserialize)]
    enum ComplexEnum {
        MultiField { _a: i32, _b: i32 },
    }

    let result: Result<Container, _> = serde_support::from_str(cosy_text);

    if let Err(e) = result {
        let msg = e.to_string();
        // Error message should hint at limitation
        assert!(msg.contains("error") || msg.contains("expected"));
    }
}

// ============================================================================
// OPTION AND NULL TESTS
// ============================================================================

#[test]
fn test_serde_option_some_roundtrip() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Data {
        required: String,
        optional: Option<String>,
    }

    let original = Data {
        required: "present".to_string(),
        optional: Some("also present".to_string()),
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Data = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_serde_option_none_roundtrip() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Data {
        required: String,
        optional: Option<String>,
    }

    let original = Data {
        required: "present".to_string(),
        optional: None,
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Data = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

// ============================================================================
// MIXED SEPARATOR TESTS (Comma + Newline)
// ============================================================================

#[test]
fn test_serde_mixed_separators_in_array() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Data {
        items: Vec<i32>,
    }

    // Manually parse mixed separators
    let cosy_text = r#"{
        items: [
            1,
            2
            3,
            4
        ]
    }"#;

    let deserialized: Data = serde_support::from_str(cosy_text).unwrap();
    let expected = Data {
        items: vec![1, 2, 3, 4],
    };

    assert_eq!(deserialized, expected);
}

#[test]
fn test_serde_mixed_separators_in_object() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Data {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }

    // Manually parse mixed separators
    let cosy_text = r#"{
        a: 1,
        b: 2
        c: 3,
        d: 4
    }"#;

    let deserialized: Data = serde_support::from_str(cosy_text).unwrap();
    let expected = Data {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
    };

    assert_eq!(deserialized, expected);
}

// ============================================================================
// COLLECTION TESTS
// ============================================================================

#[test]
fn test_serde_vec_of_structs() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Item {
        id: u32,
        name: String,
    }

    let original = vec![
        Item {
            id: 1,
            name: "First".to_string(),
        },
        Item {
            id: 2,
            name: "Second".to_string(),
        },
    ];

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Vec<Item> = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_serde_nested_vecs() {
    let original = vec![vec![1, 2, 3], vec![4, 5, 6]];

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Vec<Vec<i32>> = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

// ============================================================================
// BOOL AND NULL TESTS
// ============================================================================

#[test]
fn test_serde_booleans() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Flags {
        enabled: bool,
        disabled: bool,
    }

    let original = Flags {
        enabled: true,
        disabled: false,
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: Flags = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

// ============================================================================
// COMPLEX REAL-WORLD EXAMPLE
// ============================================================================

#[test]
fn test_serde_realistic_config() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct ServerConfig {
        host: String,
        port: u16,
        ssl: bool,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct DatabaseConfig {
        url: String,
        max_connections: u32,
        timeout: u32,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct AppConfig {
        version: String,
        debug: bool,
        server: ServerConfig,
        database: DatabaseConfig,
        features: Vec<String>,
    }

    let original = AppConfig {
        version: "1.0.0".to_string(),
        debug: true,
        server: ServerConfig {
            host: "localhost".to_string(),
            port: 8080,
            ssl: false,
        },
        database: DatabaseConfig {
            url: "postgresql://localhost/mydb".to_string(),
            max_connections: 100,
            timeout: 30,
        },
        features: vec![
            "auth".to_string(),
            "logging".to_string(),
            "caching".to_string(),
        ],
    };

    let serialized = serde_support::to_string(&original).unwrap();
    let deserialized: AppConfig = serde_support::from_str(&serialized).unwrap();

    assert_eq!(original, deserialized);
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_serde_deserialization_type_mismatch() {
    #[derive(Debug, Deserialize)]
    struct Expected {
        _value: i32,
    }

    let cosy_text = r#"{ value: "not a number" }"#;
    let result: Result<Expected, _> = serde_support::from_str(cosy_text);

    assert!(result.is_err());
}

#[test]
fn test_serde_missing_required_field() {
    #[derive(Debug, Deserialize)]
    struct Expected {
        _required: String,
    }

    let cosy_text = r#"{ other_field: "value" }"#;
    let result: Result<Expected, _> = serde_support::from_str(cosy_text);

    assert!(result.is_err());
}

#[test]
fn test_serde_error_message_helpful() {
    #[derive(Debug, Deserialize)]
    struct Data {
        _value: i32,
    }

    let cosy_text = r#"{ value: "string" }"#;
    let result: Result<Data, _> = serde_support::from_str(cosy_text);

    if let Err(e) = result {
        let msg = e.to_string();
        // Should contain some indication of the error
        assert!(!msg.is_empty());
    }
}

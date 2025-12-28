// tests/key_order_tests.rs
// Tests verifying that object key order is preserved with IndexMap

use cosy::value::{Value, ValueKind};
use cosy::{from_str, serde as serde_support, to_string};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

// ============================================================================
// BASIC KEY ORDER TESTS
// ============================================================================

#[test]
fn test_parse_preserves_key_order() {
    let input = r#"{
        first: 1
        second: 2
        third: 3
        fourth: 4
        fifth: 5
    }"#;

    let value = from_str(input).unwrap();

    if let ValueKind::Object(obj) = value.kind {
        let keys: Vec<&String> = obj.keys().collect();
        assert_eq!(keys, vec!["first", "second", "third", "fourth", "fifth"]);
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_serialize_maintains_key_order() {
    let mut obj = IndexMap::new();
    obj.insert("alpha".to_string(), Value::integer(1));
    obj.insert("bravo".to_string(), Value::integer(2));
    obj.insert("charlie".to_string(), Value::integer(3));
    obj.insert("delta".to_string(), Value::integer(4));

    let value = Value::object(obj);
    let serialized = to_string(&value);

    // Parse it back and verify order
    let reparsed = from_str(&serialized).unwrap();
    if let ValueKind::Object(obj) = reparsed.kind {
        let keys: Vec<&String> = obj.keys().collect();
        assert_eq!(keys, vec!["alpha", "bravo", "charlie", "delta"]);
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_roundtrip_preserves_order() {
    let input = r#"{
        z_last: "should be last"
        a_first: "should be first"
        m_middle: "should be middle"
    }"#;

    let parsed = from_str(input).unwrap();
    let serialized = to_string(&parsed);
    let reparsed = from_str(&serialized).unwrap();

    if let ValueKind::Object(obj) = reparsed.kind {
        let keys: Vec<&String> = obj.keys().collect();
        // Order should be preserved from original parse, not alphabetical
        assert_eq!(keys, vec!["z_last", "a_first", "m_middle"]);
    } else {
        panic!("Expected object");
    }
}

// ============================================================================
// SERDE KEY ORDER TESTS
// ============================================================================

#[test]
fn test_serde_struct_field_order_preserved() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Config {
        name: String,
        version: String,
        debug: bool,
        port: u16,
        timeout: u32,
    }

    let original = Config {
        name: "MyApp".to_string(),
        version: "1.0.0".to_string(),
        debug: true,
        port: 8080,
        timeout: 30,
    };

    let serialized = serde_support::to_string(&original).unwrap();
    println!("Serialized:\n{}\n", serialized);

    // Parse the serialized version and check key order
    let reparsed_value: Value = from_str(&serialized).unwrap();
    if let ValueKind::Object(obj) = reparsed_value.kind {
        let keys: Vec<&String> = obj.keys().collect();
        // Keys should appear in field declaration order
        assert_eq!(keys, vec!["name", "version", "debug", "port", "timeout"]);
    } else {
        panic!("Expected object");
    }

    // And verify deserialization works
    let reparsed: Config = serde_support::from_str(&serialized).unwrap();
    assert_eq!(original, reparsed);
}

#[test]
fn test_serde_nested_config_key_order() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Server {
        host: String,
        port: u16,
        ssl: bool,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Database {
        url: String,
        max_connections: u32,
        timeout: u32,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct AppConfig {
        app_name: String,
        server: Server,
        database: Database,
        debug: bool,
    }

    let original = AppConfig {
        app_name: "TestApp".to_string(),
        server: Server {
            host: "localhost".to_string(),
            port: 8080,
            ssl: false,
        },
        database: Database {
            url: "postgres://localhost/db".to_string(),
            max_connections: 100,
            timeout: 30,
        },
        debug: true,
    };

    let serialized = serde_support::to_string(&original).unwrap();
    println!("Nested config:\n{}\n", serialized);

    // Verify roundtrip
    let reparsed: AppConfig = serde_support::from_str(&serialized).unwrap();
    assert_eq!(original, reparsed);

    // Verify top-level key order
    let reparsed_value: Value = from_str(&serialized).unwrap();
    if let ValueKind::Object(obj) = reparsed_value.kind {
        let keys: Vec<&String> = obj.keys().collect();
        assert_eq!(keys, vec!["app_name", "server", "database", "debug"]);
    }
}

// ============================================================================
// INSERTION ORDER VERIFICATION
// ============================================================================

#[test]
fn test_insertion_order_not_alphabetical() {
    let input = r#"{
        zebra: 1
        apple: 2
        monkey: 3
        banana: 4
    }"#;

    let value = from_str(input).unwrap();

    if let ValueKind::Object(obj) = value.kind {
        let keys: Vec<&String> = obj.keys().collect();
        // Order should match insertion order, not alphabetical
        assert_eq!(keys, vec!["zebra", "apple", "monkey", "banana"]);
        assert_ne!(keys, vec!["apple", "banana", "monkey", "zebra"]); // Not alphabetical
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_large_object_key_order() {
    let keys_in_order = vec![
        "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i",
        "h", "g", "f", "e", "d", "c", "b", "a",
    ];

    let mut input = String::from("{");
    for (i, key) in keys_in_order.iter().enumerate() {
        if i > 0 {
            input.push(',');
        }
        input.push('\n');
        input.push_str(&format!("    {}: {}", key, i));
    }
    input.push_str("\n}");

    let value = from_str(&input).unwrap();

    if let ValueKind::Object(obj) = value.kind {
        let parsed_keys: Vec<&String> = obj.keys().collect();
        let expected_keys: Vec<&str> = keys_in_order.iter().copied().collect();
        assert_eq!(parsed_keys, expected_keys);
    } else {
        panic!("Expected object");
    }
}

// ============================================================================
// DUPLICATE KEY HANDLING (LAST ONE WINS)
// ============================================================================

#[test]
fn test_duplicate_keys_last_wins() {
    let input = r#"{
        name: "First"
        name: "Second"
        name: "Third"
    }"#;

    let value = from_str(input).unwrap();

    if let ValueKind::Object(obj) = value.kind {
        assert_eq!(obj.get("name"), Some(&Value::string("Third".to_string())));
        // Only one "name" key should exist
        assert_eq!(obj.len(), 1);
    } else {
        panic!("Expected object");
    }
}

// ============================================================================
// REAL-WORLD CONFIG EXAMPLE
// ============================================================================

#[test]
fn test_realistic_config_key_order() {
    let config_text = r#"{
        version: "1.0.0"
        name: "MyApplication"
        description: "A test application"

        server: {
            host: "0.0.0.0"
            port: 8080
            ssl: true
        }

        database: {
            url: "postgresql://localhost/mydb"
            pool_size: 10
            timeout: 30
        }

        logging: {
            level: "info"
            format: "json"
        }

        features: [
            "auth"
            "api"
            "webhooks"
        ]
    }"#;

    let value = from_str(config_text).unwrap();

    // Check top-level order
    if let ValueKind::Object(root) = value.kind {
        let keys: Vec<&String> = root.keys().collect();
        assert_eq!(
            keys,
            vec![
                "version",
                "name",
                "description",
                "server",
                "database",
                "logging",
                "features"
            ]
        );

        // Check nested object order
        if let Some(server_val) = root.get("server") {
            if let ValueKind::Object(server) = &server_val.kind {
                let server_keys: Vec<&String> = server.keys().collect();
                assert_eq!(server_keys, vec!["host", "port", "ssl"]);
            } else {
                panic!("Expected server object");
            }
        } else {
            panic!("Expected server field");
        }

        if let Some(database_val) = root.get("database") {
            if let ValueKind::Object(database) = &database_val.kind {
                let db_keys: Vec<&String> = database.keys().collect();
                assert_eq!(db_keys, vec!["url", "pool_size", "timeout"]);
            } else {
                panic!("Expected database object");
            }
        } else {
            panic!("Expected database field");
        }
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_key_order_survives_roundtrip() {
    let original = r#"{
        config_version: "2.0"
        app_name: "ProductionApp"
        environment: "production"
        logging_level: "warn"
        enable_cache: true
        max_retries: 3
    }"#;

    let value1 = from_str(original).unwrap();
    let serialized1 = to_string(&value1);

    let value2 = from_str(&serialized1).unwrap();
    let serialized2 = to_string(&value2);

    let value3 = from_str(&serialized2).unwrap();
    let serialized3 = to_string(&value3);

    // All three serializations should be identical
    assert_eq!(serialized1, serialized2);
    assert_eq!(serialized2, serialized3);

    // All three values should be equal
    assert_eq!(value1, value2);
    assert_eq!(value2, value3);

    // Key order should be consistent across all roundtrips
    if let ValueKind::Object(obj1) = value1.kind {
        if let ValueKind::Object(obj3) = value3.kind {
            let keys1: Vec<&String> = obj1.keys().collect();
            let keys3: Vec<&String> = obj3.keys().collect();
            assert_eq!(keys1, keys3);
        } else {
            panic!("value3 not object");
        }
    } else {
        panic!("value1 not object");
    }
}

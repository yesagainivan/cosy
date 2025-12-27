use cosy::{Value, load_and_merge};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_load_and_merge_basic() {
    let dir = tempdir().unwrap();
    let base_path = dir.path().join("base.cosy");
    let override_path = dir.path().join("override.cosy");

    fs::write(&base_path, r#"{ a: 1, b: 2 }"#).unwrap();
    fs::write(&override_path, r#"{ b: 3, c: 4 }"#).unwrap();

    let paths = [base_path.as_path(), override_path.as_path()];
    let config = load_and_merge(&paths).unwrap();

    if let Value::Object(map) = config {
        assert_eq!(map.get("a"), Some(&Value::Integer(1)));
        assert_eq!(map.get("b"), Some(&Value::Integer(3))); // Overridden
        assert_eq!(map.get("c"), Some(&Value::Integer(4))); // Added
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_load_and_merge_nested() {
    let dir = tempdir().unwrap();
    let p1 = dir.path().join("1.cosy");
    let p2 = dir.path().join("2.cosy");

    fs::write(&p1, r#"{ server: { host: "localhost", port: 80 } }"#).unwrap();
    fs::write(&p2, r#"{ server: { port: 443 } }"#).unwrap();

    let paths = [p1.as_path(), p2.as_path()];
    let config = load_and_merge(&paths).unwrap();

    if let Value::Object(root) = config {
        let server = root.get("server").unwrap().as_object().unwrap();
        assert_eq!(
            server.get("host"),
            Some(&Value::String("localhost".to_string()))
        );
        assert_eq!(server.get("port"), Some(&Value::Integer(443)));
    } else {
        panic!("Expected object");
    }
}

pub trait ValueExt {
    fn as_object(&self) -> Option<&indexmap::IndexMap<String, Value>>;
}

impl ValueExt for Value {
    fn as_object(&self) -> Option<&indexmap::IndexMap<String, Value>> {
        if let Value::Object(map) = self {
            Some(map)
        } else {
            None
        }
    }
}

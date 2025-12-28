use cosy::load_and_merge;
use cosy::value::{Value, ValueKind};
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

    if let ValueKind::Object(map) = config.kind {
        assert_eq!(map.get("a"), Some(&Value::integer(1)));
        assert_eq!(map.get("b"), Some(&Value::integer(3))); // Overridden
        assert_eq!(map.get("c"), Some(&Value::integer(4))); // Added
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

    if let ValueKind::Object(root) = config.kind {
        let server = root.get("server").unwrap().as_object().unwrap();
        assert_eq!(
            server.get("host"),
            Some(&Value::string("localhost".to_string()))
        );
        assert_eq!(server.get("port"), Some(&Value::integer(443)));
    } else {
        panic!("Expected object");
    }
}

pub trait ValueExt {
    fn as_object(&self) -> Option<&indexmap::IndexMap<String, Value>>;
}

impl ValueExt for Value {
    fn as_object(&self) -> Option<&indexmap::IndexMap<String, Value>> {
        match &self.kind {
            ValueKind::Object(map) => Some(map),
            _ => None,
        }
    }
}

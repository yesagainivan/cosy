use cosy::value::{Value, ValueKind};
use cosy::{from_str, include};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_basic_inclusion() {
    let dir = tempdir().unwrap();
    let base_path = dir.path().join("base.cosy");
    let app_path = dir.path().join("app.cosy");

    fs::write(
        &base_path,
        r#"{
        debug: false
        port: 8000
    }"#,
    )
    .unwrap();

    fs::write(
        &app_path,
        r#"{
        include: "base.cosy"
        debug: true
    }"#,
    )
    .unwrap();

    let app_content = fs::read_to_string(&app_path).unwrap();
    let mut config = from_str(&app_content).unwrap();

    // Resolve includes relative to the temp dir
    include::resolve(&mut config, dir.path()).unwrap();

    if let ValueKind::Object(map) = config.kind {
        assert_eq!(map.get("port"), Some(&Value::integer(8000))); // Inherited
        assert_eq!(map.get("debug"), Some(&Value::boolean(true))); // Overridden
        assert!(!map.contains_key("include")); // Removed
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_nested_inclusion() {
    let dir = tempdir().unwrap();
    let base_path = dir.path().join("base.cosy");
    let mid_path = dir.path().join("mid.cosy");
    let top_path = dir.path().join("top.cosy");

    fs::write(&base_path, r#"{ a: 1 }"#).unwrap();
    fs::write(&mid_path, r#"{ include: "base.cosy", b: 2 }"#).unwrap();
    fs::write(&top_path, r#"{ include: "mid.cosy", c: 3 }"#).unwrap();

    let top_content = fs::read_to_string(&top_path).unwrap();
    let mut config = from_str(&top_content).unwrap();

    include::resolve(&mut config, dir.path()).unwrap();

    if let ValueKind::Object(map) = config.kind {
        assert_eq!(map.get("a"), Some(&Value::integer(1)));
        assert_eq!(map.get("b"), Some(&Value::integer(2)));
        assert_eq!(map.get("c"), Some(&Value::integer(3)));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_include_in_sub_object() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("db.cosy");

    fs::write(&db_path, r#"{ host: "localhost", port: 5432 }"#).unwrap();

    // config with include inside "database" key
    let config_str = r#"{
        database: {
            include: "db.cosy"
            user: "admin"
        }
    }"#;

    let mut config = from_str(config_str).unwrap();
    include::resolve(&mut config, dir.path()).unwrap();

    if let ValueKind::Object(root) = config.kind {
        if let Some(db_val) = root.get("database") {
            if let ValueKind::Object(db) = &db_val.kind {
                assert_eq!(
                    db.get("host"),
                    Some(&Value::string("localhost".to_string()))
                );
                assert_eq!(db.get("port"), Some(&Value::integer(5432)));
                assert_eq!(db.get("user"), Some(&Value::string("admin".to_string())));
            } else {
                panic!("database should be an object");
            }
        } else {
            panic!("database field missing");
        }
    } else {
        panic!("root should be an object");
    }
}

#[test]
fn test_include_cycle_detection() {
    let dir = tempdir().unwrap();
    let a_path = dir.path().join("a.cosy");
    let b_path = dir.path().join("b.cosy");

    fs::write(&a_path, r#"{ include: "b.cosy" }"#).unwrap();
    fs::write(&b_path, r#"{ include: "a.cosy" }"#).unwrap();

    let a_content = fs::read_to_string(&a_path).unwrap();
    let mut config = from_str(&a_content).unwrap();

    let result = include::resolve(&mut config, dir.path());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Recursion limit exceeded")
    );
}

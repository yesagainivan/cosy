use cosy::value::{Value, ValueKind};
use cosy::{from_str, include};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_repro_shallow_merge() {
    let dir = tempdir().unwrap();
    let base_path = dir.path().join("base.cosy");
    let app_path = dir.path().join("app.cosy");

    // Base config: defines server with host AND port
    fs::write(
        &base_path,
        r#"{
        server: {
            host: "0.0.0.0"
            port: 8080
        }
    }"#,
    )
    .unwrap();

    // App config: overrides ONLY port
    // CURRENT BEHAVIOR (Shallow Merge): "host" will be LOST because "server" object is replaced.
    // DESIRED BEHAVIOR (Deep Merge): "host" should be PRESERVED.
    fs::write(
        &app_path,
        r#"{
        include: "base.cosy"
        server: {
            port: 9000
        }
    }"#,
    )
    .unwrap();

    let app_content = fs::read_to_string(&app_path).unwrap();
    let mut config = from_str(&app_content).unwrap();

    include::resolve(&mut config, dir.path()).unwrap();

    if let ValueKind::Object(root) = config.kind {
        if let Some(server_val) = root.get("server") {
            if let ValueKind::Object(server) = &server_val.kind {
                let host = server.get("host");
                let port = server.get("port");

                println!("Server config after merge: {:?}", server);

                assert_eq!(port, Some(&Value::integer(9000)));

                // Deep Merge: Host MUST be present
                assert_eq!(
                    host,
                    Some(&Value::string("0.0.0.0".to_string())),
                    "Expected host to be preserved"
                );
            } else {
                panic!("server should be an object");
            }
        } else {
            panic!("server field missing");
        }
    } else {
        panic!("root should be an object");
    }
}

#[test]
fn test_deep_merge_complex() {
    let dir = tempdir().unwrap();
    let base_path = dir.path().join("base.cosy");
    let app_path = dir.path().join("app.cosy");

    fs::write(
        &base_path,
        r#"{
        database: {
            host: "localhost"
            port: 5432
            options: {
                timeout: 30
                ssl: true
            }
        }
        logging: {
            level: "info"
            outputs: ["stdout"]
        }
    }"#,
    )
    .unwrap();

    // Override port, timeout, and logging level.
    // Logging outputs array should be REPLACED.
    fs::write(
        &app_path,
        r#"{
        include: "base.cosy"
        database: {
            port: 6000
            options: {
                timeout: 60
            }
        }
        logging: {
            outputs: ["file"]
        }
    }"#,
    )
    .unwrap();

    let app_content = fs::read_to_string(&app_path).unwrap();
    let mut config = from_str(&app_content).unwrap();

    include::resolve(&mut config, dir.path()).unwrap();

    if let ValueKind::Object(root) = config.kind {
        let db = root.get("database").unwrap().as_object().unwrap();

        // Host preserved
        assert_eq!(
            db.get("host"),
            Some(&Value::string("localhost".to_string()))
        );
        // Port overridden
        assert_eq!(db.get("port"), Some(&Value::integer(6000)));

        let options = db.get("options").unwrap().as_object().unwrap();
        // SSL preserved
        assert_eq!(options.get("ssl"), Some(&Value::boolean(true)));
        // Timeout overridden
        assert_eq!(options.get("timeout"), Some(&Value::integer(60)));

        let logging = root.get("logging").unwrap().as_object().unwrap();
        // Level preserved (from base)
        assert_eq!(
            logging.get("level"),
            Some(&Value::string("info".to_string()))
        );

        let outputs = logging.get("outputs").unwrap().as_array().unwrap();
        // Array REPLACED (not merged)
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0], Value::string("file".to_string()));
    } else {
        panic!("Root not object");
    }
}

pub trait ValueExt {
    fn as_object(&self) -> Option<&indexmap::IndexMap<String, Value>>;
    fn as_array(&self) -> Option<&Vec<Value>>;
}

impl ValueExt for Value {
    fn as_object(&self) -> Option<&indexmap::IndexMap<String, Value>> {
        match &self.kind {
            ValueKind::Object(map) => Some(map),
            _ => None,
        }
    }
    fn as_array(&self) -> Option<&Vec<Value>> {
        match &self.kind {
            ValueKind::Array(arr) => Some(arr),
            _ => None,
        }
    }
}

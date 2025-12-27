//! Example: Loading and Merging Multiple Config Files
//!
//! Run with: cargo run --example multi_load

use cosy::{Value, load_and_merge};
use std::fs;
use tempfile::tempdir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup: Create temporary config files simulates real environment
    let dir = tempdir()?;
    let base_path = dir.path().join("base.cosy");
    let local_path = dir.path().join("local.cosy");

    // base.cosy: Default settings
    fs::write(
        &base_path,
        r#"{
        app_name: "My App"
        server: {
            host: "0.0.0.0"
            port: 8080
        }
        debug: false
    }"#,
    )?;

    // local.cosy: Local overrides (e.g. for development)
    // Only overrides 'port' and 'debug'. 'host' and 'app_name' should be preserved.
    fs::write(
        &local_path,
        r#"{
        server: {
            port: 3000
        }
        debug: true
    }"#,
    )?;

    println!(
        "Loading configs from: {:?} and {:?}",
        base_path.file_name().unwrap(),
        local_path.file_name().unwrap()
    );

    // 1. Load and Merge
    let paths = [base_path.as_path(), local_path.as_path()];
    let config = load_and_merge(&paths)?;

    println!("\n--- Merged Configuration ---");
    // Pretty print the result
    // (In a real app, you'd deserialize this into a struct)
    if let Value::Object(root) = &config {
        println!("App Name: {}", root.get("app_name").unwrap());

        let server = root.get("server").unwrap().as_object().unwrap();
        println!(
            "Server: {{ host: {}, port: {} }}",
            server.get("host").unwrap(),
            server.get("port").unwrap()
        );

        println!("Debug Mode: {}", root.get("debug").unwrap());
    }

    Ok(())
}

trait ValueExt {
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

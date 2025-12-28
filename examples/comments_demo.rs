//! Example: Round-trip Comment Preservation
//!
//! This example demonstrates how COSY preserves comments when parsing and serializing configuration.
//!
//! Run with: cargo run --example comments_demo

use cosy::value::ValueKind;
use cosy::{from_str, to_string};

fn main() {
    let config_input = r#"
    // Global application settings
    app_name: "CosyApp"
    
    // Server configuration
    // Defines how the server listens
    server: {
        host: "localhost" // Local interface
        port: 8080        // Default port must be > 1024
    }
    
    // Feature flags
    features: [
        "logging",
        // "metrics", // Disabled for now
        "auth"
    ]
    "#;

    println!("--- Original Input ---");
    println!("{}", config_input);

    // 1. Parse the configuration
    let value = from_str(config_input).expect("Failed to parse config");

    println!("\n--- Parsed Value Structure ---");
    // We can inspect the structure to see comments are attached
    if let ValueKind::Object(root) = &value.kind {
        // Comments for "app_name" value
        if let Some(app_name) = root.get("app_name") {
            println!("Comments on 'app_name': {:?}", app_name.comments);
        }

        // Comments for "server" object
        if let Some(server) = root.get("server") {
            println!("Comments on 'server': {:?}", server.comments);
        }
    }

    // 2. Serialize it back to a string
    // The output should preserve the comments
    let output = to_string(&value);

    println!("\n--- Serialized Output ---");
    println!("{}", output);

    // Verify it matches expectations (visually)
    assert!(output.contains("// Global application settings"));
    assert!(output.contains("// Server configuration"));
    assert!(output.contains("// Local interface"));
}

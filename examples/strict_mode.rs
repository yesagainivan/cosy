//! Example: Strict Mode Validation (Typos & Deprecation)
//!
//! Run with: cargo run --example strict_mode

use cosy::schema::ValidationLevel;
use cosy::{from_str, schema};

fn main() {
    // 1. Define a schema with Strict types and Deprecation
    let schema_str = r#"{
        server: {
            host: "string"
            // Simple type
            port: "integer"
            // Deprecated field
            ssl_enabled: { type: "boolean", deprecated: "Use 'tls' block instead" }
        }
    }"#;
    let schema = from_str(schema_str).expect("Valid schema");

    // 2. Define a configuration with issues
    let config_str = r#"{
        server: {
            host: "localhost"
            // TYPO: 'prt' instead of 'port'
            prt: 8080
            // DEPRECATED usage
            ssl_enabled: true
        }
    }"#;
    let config = from_str(config_str).expect("Parsed config");

    println!("--- Validating Config ---");

    // 3. Validate
    match schema::validate(&config, &schema) {
        Ok(report) => {
            if report.is_empty() {
                println!("✅ Config is valid!");
            } else {
                println!("⚠️  Validation found issues:");
                for item in report {
                    match item.level {
                        ValidationLevel::Error => {
                            println!("❌ Error: {} (at {})", item.message, item.path)
                        }
                        ValidationLevel::Warning => {
                            println!("🔸 Warning: {} (at {})", item.message, item.path)
                        }
                    }
                }
            }
        }
        Err(e) => {
            // Validation itself failed (critical error, e.g. invalid schema format)
            println!("Fatal Validation Error: {}", e);
        }
    }
}

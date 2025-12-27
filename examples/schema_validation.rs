use cosy::{from_str, schema};

fn main() {
    // 1. Define the Schema (using COSY syntax)
    let schema_text = r#"{
        server: {
            host: "string"
            port: "integer"
            enabled: "boolean"
        }
        endpoints: ["string"]
    }"#;
    let schema = from_str(schema_text).expect("Failed to parse schema");

    println!("--- Schema ---");
    println!("{}", schema_text);

    // 2. Valid Configuration
    let valid_config = r#"{
        server: {
            host: "localhost"
            port: 8080
            enabled: true
        }
        endpoints: ["/api/v1", "/health"]
    }"#;
    let config = from_str(valid_config).expect("Failed to parse config");

    println!("\n--- Validating Correct Config ---");
    match schema::validate(&config, &schema) {
        Ok(_) => println!("✅ Validation passed!"),
        Err(e) => println!("❌ Validation failed: {}", e),
    }

    // 3. Invalid Configuration (Type Mismatch)
    let invalid_config = r#"{
        server: {
            host: "localhost"
            port: "8080" // Error: Expected integer, got string
            enabled: true
        }
        endpoints: []
    }"#;
    let bad_config = from_str(invalid_config).expect("Failed to parse invalid config");

    println!("\n--- Validating Incorrect Config ---");
    match schema::validate(&bad_config, &schema) {
        Ok(_) => println!("✅ Validation passed!"),
        Err(e) => println!("❌ Validation failed: {}", e),
    }
}

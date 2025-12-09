use cosy::from_str;

fn main() {
    let cosy_config = r#"{
        // Server configuration
        server: {
            host: "localhost"
            port: 8080
            ssl: false
        }

        // Database settings
        database: {
            url: "postgresql://localhost/mydb"
            max_connections: 100
            timeout: 30
        }

        // Feature flags
        features: [
            "auth"
            "logging"
            "caching"
        ]

        // Metadata
        version: "1.0.0"
        debug: true
    }"#;

    match from_str(cosy_config) {
        Ok(value) => {
            println!("✓ Successfully parsed COSY configuration");
            println!("\n{:#?}", value);
        }
        Err(e) => {
            eprintln!(
                "✗ Parse error at line {}, column {}: {}",
                e.line(),
                e.column(),
                e.message()
            );
            std::process::exit(1);
        }
    }
}

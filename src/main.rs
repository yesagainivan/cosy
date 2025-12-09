use cosy::{SerializeOptions, from_str, to_string};

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

    println!("=== PARSING ===\n");
    match from_str(cosy_config) {
        Ok(value) => {
            println!("✓ Successfully parsed COSY configuration\n");

            println!("=== SERIALIZING (Pretty) ===\n");
            let pretty = to_string(&value);
            println!("{}\n", pretty);

            println!("=== SERIALIZING (Compact) ===\n");
            let options = SerializeOptions {
                use_newlines: false,
                trailing_commas: false,
                indent_size: 2,
            };
            let compact = cosy::to_string_with_options(&value, options);
            println!("{}\n", compact);

            println!("=== ROUNDTRIP TEST ===\n");
            match from_str(&pretty) {
                Ok(reparsed) => {
                    if reparsed == value {
                        println!(
                            "✓ Roundtrip successful! Parse → Serialize → Parse works perfectly."
                        );
                    } else {
                        println!("✗ Values differ after roundtrip");
                    }
                }
                Err(e) => {
                    eprintln!("✗ Failed to reparse serialized output: {}", e);
                }
            }
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

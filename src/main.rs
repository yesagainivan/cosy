use cosy::serde as cosy_serde;
use serde::{Deserialize, Serialize};

/// Define your configuration structure with Serde derives
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ServerConfig {
    host: String,
    port: u16,
    ssl: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
    timeout: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct AppConfig {
    version: String,
    debug: bool,
    server: ServerConfig,
    database: DatabaseConfig,
    features: Vec<String>,
}

fn main() {
    let cosy_text = r#"{
        version: "1.0.0"
        debug: true

        server: {
            host: "localhost"
            port: 8080
            ssl: false
        }

        database: {
            url: "postgresql://localhost/mydb"
            max_connections: 100
            timeout: 30
        }

        features: [
            "auth"
            "logging"
            "caching"
        ]
    }"#;

    println!("=== SERDE DESERIALIZATION ===\n");

    match cosy_serde::from_str::<AppConfig>(cosy_text) {
        Ok(config) => {
            println!("✓ Successfully deserialized into AppConfig struct!\n");
            println!("Config: {:#?}\n", config);

            println!("=== TYPE-SAFE ACCESS ===\n");
            println!("Version: {}", config.version);
            println!("Debug mode: {}", config.debug);
            println!("Server: {}:{}", config.server.host, config.server.port);
            println!("Database: {}", config.database.url);
            println!("Features: {:?}\n", config.features);

            println!("=== SERDE SERIALIZATION ===\n");
            match cosy_serde::to_string(&config) {
                Ok(serialized) => {
                    println!("✓ Successfully serialized back to COSY:\n");
                    println!("{}\n", serialized);

                    println!("=== ROUNDTRIP TEST ===\n");
                    match cosy_serde::from_str::<AppConfig>(&serialized) {
                        Ok(reparsed) => {
                            if reparsed == config {
                                println!(
                                    "✓ Perfect roundtrip! Struct → COSY → Struct preserves all data"
                                );
                            } else {
                                println!("✗ Roundtrip failed: configs differ");
                            }
                        }
                        Err(e) => {
                            eprintln!("✗ Failed to reparse: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("✗ Serialization failed: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!(
                "✗ Deserialization failed at line {}, column {}: {}",
                e.line(),
                e.column(),
                e.message()
            );
            std::process::exit(1);
        }
    }
}

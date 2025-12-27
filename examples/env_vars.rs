use cosy::serde::from_str;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
    api_key: String,
    debug_mode: bool,
}

fn main() {
    // 1. Setup environment variables (simulating a real production environment)
    unsafe {
        env::set_var("HOST", "127.0.0.1");
        env::set_var("PORT", "9090"); // Will be parsed as integer!
        env::set_var("API_KEY", "secret_12345");
        env::set_var("DEBUG", "true"); // Will be parsed as boolean!
    }

    // 2. Define config with interpolation syntax
    let config_text = r#"{
        // String interpolation
        host: "${HOST}"
        
        // Standalone interpolation (type inference)
        port: ${PORT}
        debug_mode: ${DEBUG}
        
        // Mixed string
        api_key: "key-${API_KEY}"
    }"#;

    println!("--- Parsing Config with Env Vars ---");
    println!("Environment:");
    println!("HOST=127.0.0.1, PORT=9090, DEBUG=true, API_KEY=secret_12345");
    println!("\nConfig Text:");
    println!("{}", config_text);

    // 3. Parse
    let config: ServerConfig = from_str(config_text).expect("Failed to parse config");

    println!("\n--- Parsed Struct ---");
    println!("{:#?}", config);

    // Cleanup
    unsafe {
        env::remove_var("HOST");
        env::remove_var("PORT");
        env::remove_var("API_KEY");
        env::remove_var("DEBUG");
    }
}

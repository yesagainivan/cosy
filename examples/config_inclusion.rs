use cosy::{Value, from_str, include};
use std::fs;
use std::path::Path;

fn main() {
    // 1. Create a dummy "base.cosy" file
    let base_config = r#"{
        app_name: "Cosy App"
        version: "1.0.0"
        debug: false
        server: {
            port: 8080
            host: "localhost"
        }
    }"#;
    fs::write("base_example.cosy", base_config).expect("Failed to write base config");

    // 2. Create an "app.cosy" file that includes "base.cosy"
    let app_config = r#"{
        include: "base_example.cosy"
        
        // Override debug mode
        debug: true
        
        // Add new fields
        theme: "dark"
        
        // Update nested port
        server: {
            port: 9000
        }
    }"#;
    fs::write("app_example.cosy", app_config).expect("Failed to write app config");

    println!("--- Base Config (base_example.cosy) ---");
    println!("{}", base_config);

    println!("\n--- App Config (app_example.cosy) ---");
    println!("{}", app_config);

    // 3. Parse and Resolve
    let mut config: Value = from_str(app_config).expect("Failed to parse config");

    println!("\n--- Resolving Includes... ---");
    include::resolve(&mut config, Path::new(".")).expect("Failed to resolve includes");

    println!("\n--- Final Merged Config ---");
    println!("{:#}", config); // Using alternate print for pretty indentation if impl... wait Value impl Display? 
    // My Value Display impl does not pretty print with indentation yet, it just prints.
    // But Debug does.
    println!("{:#?}", config);

    // Cleanup
    fs::remove_file("base_example.cosy").ok();
    fs::remove_file("app_example.cosy").ok();
}

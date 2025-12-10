use cosy::serde_support;
use serde::{Deserialize, Serialize};

// Enum to showcase unit and newtype variant support
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
enum LogLevel {
    Error,         // Unit variant
    Warn,          // Unit variant
    Info,          // Unit variant
    Trace(String), // Newtype variant (e.g., Trace("ModuleA"))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct MetricsConfig {
    // Unquoted key in COSY
    enabled: bool,
    // Float value
    sample_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct AppConfig {
    name: String,
    // Optional field (serializes to null if None)
    admin_email: Option<String>,
    log_level: LogLevel,
    metrics: MetricsConfig,
    // Array with newlines as separators
    endpoints: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let cosy_text = r#"{
    //     // Main application config
    //     name: "Telemetry Service"
    //     admin_email: null // Explicitly null/None
    //     log_level: { Trace: "Startup" } // Newtype enum variant, wrapped in an object

    //     metrics: {
    //         enabled: true
    //         sample_rate: 0.05e-1 // Float number
    //     }

    //     // Array with newlines as separators, no commas needed
    //     endpoints: [
    //         "/api/v1"
    //         "/api/v2/beta"
    //         "/metrics"
    //     ]
    //     // Trailing comma is allowed
    // }"#;

    let cosy_text = std::fs::read_to_string("config.cosy")?;
    // println!("cos:\n {}", cosy_text);

    println!("=== COSY DESERIALIZATION (Advanced) ===\n");

    // 1. Deserialize the complex structure
    let config: AppConfig = serde_support::from_str(cosy_text.as_str())?;

    println!("✓ Successfully deserialized into AppConfig struct!");
    println!("Config: {:#?}\n", config);

    println!("=== TYPE-SAFE ACCESS ===\n");
    println!("Service: {}", config.name);
    println!("Admin Email: {:?}", config.admin_email); // Will print None
    println!("Log Level: {:?}\n", config.log_level);

    // 2. Modify the struct
    let mut modified_config = config.clone();
    modified_config.log_level = LogLevel::Error;
    modified_config.metrics.sample_rate = 1.0;
    modified_config.admin_email = Some("contact@example.com".to_string());

    // 3. Serialize back to COSY
    println!("=== COSY SERIALIZATION (Modified) ===\n");
    let serialized = serde_support::to_string(&modified_config)?;

    println!("✓ Successfully serialized back to COSY:\n");
    println!("{}\n", serialized);

    // 4. Test the Serde error handling
    let bad_cosy = r#"{ log_level: 123 }"#;
    match serde_support::from_str::<AppConfig>(bad_cosy) {
        Ok(_) => {}
        Err(e) => {
            println!("=== COSY ERROR REPORTING ===\n");
            eprintln!("✗ Expected error captured: {}", e.message());
        }
    }

    Ok(())
}

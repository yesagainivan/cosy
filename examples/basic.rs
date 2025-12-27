use cosy::serde::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct UserConfig {
    username: String,
    theme: String,
    notifications_enabled: bool,
    retry_count: u32,
}

fn main() {
    let config_text = r#"{
        username: "cosy_user"
        theme: "dark"
        // Implicit boolean support
        notifications_enabled: true
        retry_count: 3
    }"#;

    println!("--- Parsing Basic Config ---");
    println!("{}", config_text);

    let config: UserConfig = from_str(config_text).expect("Failed to parse config");

    println!("\n--- Parsed Struct ---");
    println!("{:#?}", config);

    println!("\n--- Usage ---");
    println!("Username: {}", config.username);
    println!("Theme: {}", config.theme);
    println!("Notifications enabled: {}", config.notifications_enabled);
    println!("Retry count: {}", config.retry_count);
}

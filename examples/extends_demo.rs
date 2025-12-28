use cosy::{Value, from_str, include};
use std::fs;
use std::path::Path;

fn main() {
    println!("--- Cosy Extends Demo ---\n");

    // 1. Create a "base_theme.cosy"
    // This defines our defaults.
    let base_theme = r#"{
        name: "Base Theme"
        colors: {
            primary: "blue"
            secondary: "gray"
            background: "white"
        }
        spacing: {
            unit: 4
            padding: 16
        }
    }"#;
    fs::write("base_theme.cosy", base_theme).expect("Failed to write base theme");

    // 2. Create a "mixin_colors.cosy"
    // This is a partial config we might want to include to override just colors.
    let mixin_colors = r#"{
        colors: {
            secondary: "silver"
            accent: "teal"
        }
    }"#;
    fs::write("mixin_colors.cosy", mixin_colors).expect("Failed to write mixin");

    // 3. Create "my_app_theme.cosy" using `extends`
    // It extends base, includes mixin, and manages local overrides.
    // Order: Local > Mixin > Extends (Base)
    let app_theme = r#"{
        extends: "base_theme.cosy"
        include: "mixin_colors.cosy"
        
        // Local override
        name: "My App Theme"
        
        colors: {
            primary: "purple" // Overrides base 'blue'
            // secondary comes from mixin ('silver') which overrides base ('gray')
            // background comes from base ('white')
        }
    }"#;
    fs::write("my_app_theme.cosy", app_theme).expect("Failed to write app theme");

    println!("Loading 'my_app_theme.cosy'...");

    // 4. Parse and Resolve
    let mut config: Value = from_str(app_theme).expect("Failed to parse config");

    include::resolve(&mut config, Path::new(".")).expect("Failed to resolve includes/extends");

    println!("Resolving finished. Result:\n");

    // We can't easily pretty-print with the current Display impl, so let's inspect fields manually
    // or rely on Debug print.
    if let cosy::value::ValueKind::Object(map) = &config.kind {
        println!("Name: {}", map.get("name").unwrap());

        let colors = map.get("colors").unwrap();
        if let cosy::value::ValueKind::Object(cmap) = &colors.kind {
            println!("Colors:");
            println!(
                "  Primary:    {} (Local override)",
                cmap.get("primary").unwrap()
            );
            println!(
                "  Secondary:  {} (From Mixin)",
                cmap.get("secondary").unwrap()
            );
            println!(
                "  Background: {} (From Base)",
                cmap.get("background").unwrap()
            );
            println!("  Accent:     {} (From Mixin)", cmap.get("accent").unwrap());
        }
    }

    // Cleanup
    fs::remove_file("base_theme.cosy").ok();
    fs::remove_file("mixin_colors.cosy").ok();
    fs::remove_file("my_app_theme.cosy").ok();
}

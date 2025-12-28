use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "check" => {
            if args.len() < 3 {
                eprintln!("Error: Missing file path for 'check' command.");
                print_usage();
                process::exit(1);
            }
            check_file(&args[2]);
        }
        "help" | "--help" | "-h" => {
            print_usage();
        }
        cmd => {
            eprintln!("Error: Unknown command '{}'", cmd);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("COSY - Comfortable Object Syntax, Yay!");
    println!("\nUsage:");
    println!("  cosy check <file>   Parse and validate a file syntax");
    println!("  cosy help           Show this help message");
}

fn check_file(path: &str) {
    println!("Checking '{}'...", path);

    match fs::read_to_string(path) {
        Ok(content) => match cosy::from_str(&content) {
            Ok(_) => {
                println!("✅ Syntax OK");
            }
            Err(e) => {
                eprintln!("❌ Parse Error: {}", e);
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("❌ IO Error: Failed to read file '{}': {}", path, e);
            process::exit(1);
        }
    }
}

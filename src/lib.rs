//! # COSY: Comfortable Object Syntax, Yay!
//!
//! A human-friendly configuration format built in Rust with full Serde support.
//!
//! ## Key Features
//!
//! - **Human-Centric**: Comments (`//`), unquoted keys, and optional commas.
//! - **Robust**: Type distinction (integers vs floats), exact error reporting.
//! - **Composable**: Config inclusion (`include: "base.cosy"`) and merging.
//! - **Safe**: Schema validation and strict mode for typo detection.
//! - **Serde Integration**: Seamlessly map config files to Rust structs.
//!
//! ## Example
//!
//! ```no_run
//! use serde::Deserialize;
//! use cosy::serde::from_str;
//!
//! #[derive(Deserialize)]
//! struct Config {
//!     server_name: String,
//!     port: u16,
//!     debug: bool,
//! }
//!
//! let config_str = r#"{
//!     server_name: "MyServer"
//!     port: 8080
//!     debug: true  // dev mode
//! }"#;
//!
//! let config: Config = from_str(config_str).unwrap();
//! ```

// --- Modules ---

pub mod error;
pub mod include;
pub mod load;
pub mod merge;
pub mod schema;
pub mod serde;
pub mod syntax;
pub mod value;

// --- Prelude / Re-exports ---

// Primary types
pub use error::CosynError;
pub use value::Value;

// Parsing
pub use syntax::parser::{ParseError, from_str};

// Convenience utilities
pub use load::load_and_merge;
pub use serde::serializer::{SerializeOptions, to_string, to_string_with_options};

// Feature re-exports
pub use include::resolve as resolve_includes;
pub use merge::merge;
pub use schema::validate;

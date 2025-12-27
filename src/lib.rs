//! # COSY: Comfortable Object Syntax, Yay!
//!
//! A human-friendly configuration format built in Rust with full Serde support.
//!
//! ## Features
//!
//! - **Comments**: `// This is a comment`
//! - **Unquoted keys**: `name: "Alice"` (no quotes needed around `name`)
//! - **Trailing commas**: `[1, 2, 3,]` (allowed everywhere)
//! - **Newlines as separators**: Objects and arrays can use newlines instead of commas
//! - **Type distinction**: Separate integers from floats, proper null support
//! - **Detailed error messages**: Accurate line/column information
//! - **Full Serde support**: Automatic serialization/deserialization to Rust structs
//! - **Preserved key order**: Object keys maintain insertion order
//!
//! ## Example with Serde
//!
//! ```no_run
//! use serde::{Deserialize, Serialize};
//! use cosy;
//!
//! #[derive(Serialize, Deserialize)]
//! struct Config {
//!     name: String,
//!     age: u32,
//!     scores: Vec<i32>,
//! }
//!
//! let cosy_text = r#"{
//!     name: "Alice"
//!     age: 30
//!     scores: [95, 87, 92]
//! }"#;
//!
//! // Direct deserialization into your struct!
//! let config: Config = cosy::serde::from_str(cosy_text).unwrap();
//! assert_eq!(config.name, "Alice");
//! assert_eq!(config.age, 30);
//!
//! // And serialize back
//! let serialized = cosy::serde::to_string(&config).unwrap();
//! println!("{}", serialized);
//! ```

pub mod error;
pub mod include;
pub mod load;
pub mod merge;
pub mod schema;
pub mod serde;
pub mod syntax;
pub mod value;

pub use error::CosynError;
pub use syntax::parser::{ParseError, from_str};
pub use value::Value;

// Re-export Serde support for backward compatibility if desired, or point to new paths
// The previous serializer exports:
pub use load::load_and_merge;
pub use serde::serializer::{SerializeOptions, to_string, to_string_with_options};

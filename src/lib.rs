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
//! let config: Config = cosy::serde_support::from_str(cosy_text).unwrap();
//! assert_eq!(config.name, "Alice");
//! assert_eq!(config.age, 30);
//!
//! // And serialize back
//! let serialized = cosy::serde_support::to_string(&config).unwrap();
//! println!("{}", serialized);
//! ```

pub mod lexer;
pub mod parser;
pub mod serde_support;
pub mod serializer;

use indexmap::IndexMap;
use std::fmt;

/// COSY Value type - the core data structure representing any COSY value.
///
/// This enum covers all possible values in COSY:
/// - Null values
/// - Booleans
/// - Integers (distinct from floats)
/// - Floating-point numbers
/// - UTF-8 strings
/// - Arrays of values
/// - Objects (key-value maps with insertion-order preservation)
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Null value
    Null,
    /// Boolean value
    Bool(bool),
    /// 64-bit signed integer
    Integer(i64),
    /// 64-bit floating-point number
    Float(f64),
    /// UTF-8 string
    String(String),
    /// Homogeneous array of values
    Array(Vec<Value>),
    /// Object (map) with string keys, preserving insertion order
    Object(IndexMap<String, Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Object(obj) => {
                write!(f, "{{")?;
                for (i, (k, v)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

/// Unified error type for COSY parsing.
///
/// This wraps both lexical and parsing errors, providing a single error type
/// to handle all failure cases. Use the helper methods to access error details.
#[derive(Debug, Clone)]
pub enum CosynError {
    /// A lexical error (tokenization failed)
    Lex(lexer::LexError),
    /// A parsing error (tokens couldn't be converted to a value)
    Parse(parser::ParseError),
}

impl fmt::Display for CosynError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CosynError::Lex(e) => write!(f, "{}", e),
            CosynError::Parse(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for CosynError {}

impl From<lexer::LexError> for CosynError {
    fn from(e: lexer::LexError) -> Self {
        CosynError::Lex(e)
    }
}

impl From<parser::ParseError> for CosynError {
    fn from(e: parser::ParseError) -> Self {
        CosynError::Parse(e)
    }
}

impl CosynError {
    /// Get the line number where the error occurred.
    pub fn line(&self) -> usize {
        match self {
            CosynError::Lex(e) => e.line,
            CosynError::Parse(e) => e.line,
        }
    }

    /// Get the column number where the error occurred.
    pub fn column(&self) -> usize {
        match self {
            CosynError::Lex(e) => e.column,
            CosynError::Parse(e) => e.column,
        }
    }

    /// Get the error message.
    pub fn message(&self) -> &str {
        match self {
            CosynError::Lex(e) => &e.message,
            CosynError::Parse(e) => &e.message,
        }
    }
}

// Re-export public API
pub use parser::{ParseError, from_str};
pub use serializer::{SerializeOptions, to_string, to_string_with_options};

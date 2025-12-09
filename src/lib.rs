pub mod lexer;
pub mod parser;

use std::collections::HashMap;
use std::fmt;

/// COSY Value type - the core data structure
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
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

/// Unified COSY error type that wraps lexer and parser errors
#[derive(Debug, Clone)]
pub enum CosynError {
    Lex(lexer::LexError),
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
    /// Get line number if available
    pub fn line(&self) -> usize {
        match self {
            CosynError::Lex(e) => e.line,
            CosynError::Parse(e) => e.line,
        }
    }

    /// Get column number if available
    pub fn column(&self) -> usize {
        match self {
            CosynError::Lex(e) => e.column,
            CosynError::Parse(e) => e.column,
        }
    }

    /// Get error message
    pub fn message(&self) -> &str {
        match self {
            CosynError::Lex(e) => &e.message,
            CosynError::Parse(e) => &e.message,
        }
    }
}

// Re-export public API
pub use parser::{ParseError, from_str};

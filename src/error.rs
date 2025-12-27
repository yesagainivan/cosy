use crate::syntax::{lexer, parser};
use std::fmt;

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

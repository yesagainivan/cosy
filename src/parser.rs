use crate::lexer::{Lexer, Position, Token, TokenWithPos};
use crate::{CosynError, Value};
use indexmap::IndexMap;
use std::error::Error;
use std::fmt;

/// Parser error with detailed position information
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Parse error at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl Error for ParseError {}

/// The COSY parser with position tracking
pub struct Parser {
    tokens: Vec<TokenWithPos>,
    position: usize,
}

impl Parser {
    /// Create a new parser from tokens
    pub fn new(tokens: Vec<TokenWithPos>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    /// Parse a complete COSY document
    pub fn parse(&mut self) -> Result<Value, ParseError> {
        self.skip_newlines(); // Allow leading newlines
        let value = self.parse_value()?;

        self.skip_newlines(); // Allow trailing newlines

        // Ensure we've consumed all tokens (EOF should be next)
        if !matches!(self.current_token(), Token::Eof) {
            return Err(self.error_at_current("Unexpected tokens after value".to_string()));
        }

        Ok(value)
    }

    /// Parse any value
    fn parse_value(&mut self) -> Result<Value, ParseError> {
        match &self.current_token() {
            Token::Null => {
                self.advance();
                Ok(Value::Null)
            }
            Token::True => {
                self.advance();
                Ok(Value::Bool(true))
            }
            Token::False => {
                self.advance();
                Ok(Value::Bool(false))
            }
            Token::Integer(i) => {
                let val = Value::Integer(*i);
                self.advance();
                Ok(val)
            }
            Token::Float(f) => {
                let val = Value::Float(*f);
                self.advance();
                Ok(val)
            }
            Token::String(s) => {
                let val = Value::String(s.clone());
                self.advance();
                Ok(val)
            }
            Token::LeftBrace => self.parse_object(),
            Token::LeftBracket => self.parse_array(),
            token => Err(self.error_at_current(format!("Expected value, found {}", token))),
        }
    }

    /// Expect a specific token, advance if found
    fn expect(&mut self, expected: Token, message: &str) -> Result<(), ParseError> {
        let current = self.current_token();
        let matches = match (&current, &expected) {
            (Token::LeftBrace, Token::LeftBrace) => true,
            (Token::RightBrace, Token::RightBrace) => true,
            (Token::LeftBracket, Token::LeftBracket) => true,
            (Token::RightBracket, Token::RightBracket) => true,
            (Token::Colon, Token::Colon) => true,
            _ => false,
        };

        if matches {
            self.advance();
            Ok(())
        } else {
            Err(self.error_at_current(message.to_string()))
        }
    }

    /// Current token
    fn current_token(&self) -> Token {
        if self.is_at_end() {
            Token::Eof
        } else {
            self.tokens[self.position].token.clone()
        }
    }

    /// Current position
    fn current_position(&self) -> Position {
        if self.is_at_end() {
            Position::new(1, 1)
        } else {
            self.tokens[self.position].pos
        }
    }

    /// Advance to next token
    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
        }
    }

    /// Check if we're at the end
    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    /// Create an error at current position
    fn error_at_current(&self, message: String) -> ParseError {
        let pos = self.current_position();
        ParseError {
            message,
            line: pos.line,
            column: pos.column,
        }
    }

    /// Skip any number of newlines, returning true if any were skipped
    fn skip_newlines(&mut self) -> bool {
        let mut skipped = false;
        while matches!(self.current_token(), Token::Newline) {
            self.advance();
            skipped = true;
        }
        skipped
    }

    /// Parse an object with optional commas after newlines
    fn parse_object(&mut self) -> Result<Value, ParseError> {
        self.expect(Token::LeftBrace, "Expected '{' to start object")?;

        let mut object = IndexMap::new();

        loop {
            self.skip_newlines();

            // Handle empty object or end of object
            if matches!(self.current_token(), Token::RightBrace) {
                self.advance();
                return Ok(Value::Object(object));
            }

            // Parse key (identifier or string)
            let key = match &self.current_token() {
                Token::Identifier(s) => {
                    let k = s.clone();
                    self.advance();
                    k
                }
                Token::String(s) => {
                    let k = s.clone();
                    self.advance();
                    k
                }
                token => {
                    return Err(self.error_at_current(format!(
                        "Expected object key (identifier or string), found {}",
                        token
                    )));
                }
            };

            // Parse colon
            self.expect(Token::Colon, "Expected ':' after object key")?;

            // Parse value
            let value = self.parse_value()?;
            object.insert(key, value);

            // Check for separator (comma or newline)
            let mut has_sep = self.skip_newlines();

            if matches!(self.current_token(), Token::Comma) {
                self.advance();
                has_sep = true;
                self.skip_newlines();
            }

            if matches!(self.current_token(), Token::RightBrace) {
                self.advance();
                break;
            }

            if !has_sep {
                return Err(self.error_at_current(format!(
                    "Expected ',' or '}}' in object, found {}",
                    self.current_token()
                )));
            }
        }

        Ok(Value::Object(object))
    }

    /// Parse an array with optional commas after newlines
    fn parse_array(&mut self) -> Result<Value, ParseError> {
        self.expect(Token::LeftBracket, "Expected '[' to start array")?;

        let mut array = Vec::new();

        loop {
            self.skip_newlines();

            // Handle empty array or end of array
            if matches!(self.current_token(), Token::RightBracket) {
                self.advance();
                return Ok(Value::Array(array));
            }

            // Parse value
            let value = self.parse_value()?;
            array.push(value);

            // Check for separator
            let mut has_sep = self.skip_newlines();

            if matches!(self.current_token(), Token::Comma) {
                self.advance();
                has_sep = true;
                self.skip_newlines();
            }

            if matches!(self.current_token(), Token::RightBracket) {
                self.advance();
                break;
            }

            if !has_sep {
                return Err(self.error_at_current(format!(
                    "Expected ',' or ']' in array, found {}",
                    self.current_token()
                )));
            }
        }

        Ok(Value::Array(array))
    }
}

/// Parse COSY from a string
pub fn from_str(input: &str) -> Result<Value, CosynError> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?; // ? operator converts LexError to CosynError
    let mut parser = Parser::new(tokens);
    let value = parser.parse()?; // ? operator converts ParseError to CosynError
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_null() {
        let tokens = vec![
            TokenWithPos::new(Token::Null, Position::new(1, 1)),
            TokenWithPos::new(Token::Eof, Position::new(1, 5)),
        ];
        let mut parser = Parser::new(tokens);
        let value = parser.parse().unwrap();
        assert_eq!(value, Value::Null);
    }

    #[test]
    fn test_error_has_position() {
        let tokens = vec![
            TokenWithPos::new(Token::Integer(42), Position::new(2, 10)),
            TokenWithPos::new(Token::Integer(99), Position::new(2, 13)),
            TokenWithPos::new(Token::Eof, Position::new(2, 16)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.line, 2);
        assert_eq!(err.column, 13);
    }

    #[test]
    fn test_integration_with_lexer() {
        let input = r#"{
        name: "Alice"
        age: 30
        scores: [95, 87, 92]
    }"#;
        let value = from_str(input).unwrap();

        match value {
            Value::Object(obj) => {
                assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
                assert_eq!(obj.get("age"), Some(&Value::Integer(30)));
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_key_order_preservation() {
        let input = r#"{
        first: 1
        second: 2
        third: 3
        fourth: 4
    }"#;
        let value = from_str(input).unwrap();

        match value {
            Value::Object(obj) => {
                let keys: Vec<&String> = obj.keys().collect();
                assert_eq!(keys, vec!["first", "second", "third", "fourth"]);
            }
            _ => panic!("Expected object"),
        }
    }
}

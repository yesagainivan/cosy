use crate::Value;
use crate::lexer::{Lexer, Position, Token, TokenWithPos};
use std::collections::HashMap;
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
        let value = self.parse_value()?;

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
}

/// Parser changes to handle optional commas after newlines

// In parser.rs, add this helper method:

impl Parser {
    /// Check if there's a newline before current token
    fn has_newline_before_current(&self) -> bool {
        if self.position == 0 {
            return false;
        }

        let current_line = self.current_position().line;
        let prev_line = self.tokens[self.position - 1].pos.line;

        current_line > prev_line
    }

    /// Parse an object with optional commas after newlines
    fn parse_object(&mut self) -> Result<Value, ParseError> {
        self.expect(Token::LeftBrace, "Expected '{' to start object")?;

        let mut object = HashMap::new();

        // Handle empty object
        if matches!(self.current_token(), Token::RightBrace) {
            self.advance();
            return Ok(Value::Object(object));
        }

        loop {
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

            // Check for comma, newline, or end of object
            match &self.current_token() {
                Token::Comma => {
                    self.advance();
                    // Allow trailing comma before }
                    if matches!(self.current_token(), Token::RightBrace) {
                        self.advance();
                        break;
                    }
                }
                Token::RightBrace => {
                    self.advance();
                    break;
                }
                _ if self.has_newline_before_current() => {
                    // Newline acts as separator, continue parsing next key
                    continue;
                }
                token => {
                    return Err(self.error_at_current(format!(
                        "Expected ',' or '}}' in object, found {}",
                        token
                    )));
                }
            }
        }

        Ok(Value::Object(object))
    }

    /// Parse an array with optional commas after newlines
    fn parse_array(&mut self) -> Result<Value, ParseError> {
        self.expect(Token::LeftBracket, "Expected '[' to start array")?;

        let mut array = Vec::new();

        // Handle empty array
        if matches!(self.current_token(), Token::RightBracket) {
            self.advance();
            return Ok(Value::Array(array));
        }

        loop {
            // Parse value
            let value = self.parse_value()?;
            array.push(value);

            // Check for comma, newline, or end of array
            match &self.current_token() {
                Token::Comma => {
                    self.advance();
                    // Allow trailing comma before ]
                    if matches!(self.current_token(), Token::RightBracket) {
                        self.advance();
                        break;
                    }
                }
                Token::RightBracket => {
                    self.advance();
                    break;
                }
                _ if self.has_newline_before_current() => {
                    // Newline acts as separator, continue parsing next value
                    continue;
                }
                token => {
                    return Err(self.error_at_current(format!(
                        "Expected ',' or ']' in array, found {}",
                        token
                    )));
                }
            }
        }

        Ok(Value::Array(array))
    }
}

/// Parse COSY from a string
pub fn from_str(input: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let value = parser.parse()?;
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
}

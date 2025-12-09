use crate::Value;
use crate::lexer::{Lexer, Token};
use std::error::Error;

use std::collections::HashMap;
use std::fmt; // Import the Error trait

/// Parser error with context
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

// Implement std::error::Error for ParseError
impl Error for ParseError {}

/// The COSY parser
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Create a new parser from tokens
    pub fn new(tokens: Vec<Token>) -> Self {
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
            return Err(self.error("Unexpected tokens after value".to_string()));
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
            token => Err(self.error(format!("Expected value, found {}", token))),
        }
    }

    /// Parse an object: { key: value, key: value, ... }
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
                    return Err(self.error(format!(
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

            // Check for comma or end of object
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
                token => {
                    return Err(
                        self.error(format!("Expected ',' or '}}' in object, found {}", token))
                    );
                }
            }
        }

        Ok(Value::Object(object))
    }

    /// Parse an array: [ value, value, ... ]
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

            // Check for comma or end of array
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
                token => {
                    return Err(
                        self.error(format!("Expected ',' or ']' in array, found {}", token))
                    );
                }
            }
        }

        Ok(Value::Array(array))
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
            Err(self.error(message.to_string()))
        }
    }

    /// Current token
    fn current_token(&self) -> Token {
        if self.is_at_end() {
            Token::Eof
        } else {
            self.tokens[self.position].clone()
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
    fn error(&self, message: String) -> ParseError {
        ParseError {
            message,
            line: 1, // TODO: Track line/column from tokens
            column: 1,
        }
    }
}

/// Parse COSY from a string
pub fn from_str(input: &str) -> Result<Value, Box<dyn std::error::Error>> {
    // Assuming LexError in lexer.rs will also implement std::error::Error
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
        let tokens = vec![Token::Null, Token::Eof];
        let mut parser = Parser::new(tokens);
        let value = parser.parse().unwrap();
        assert_eq!(value, Value::Null);
    }

    #[test]
    fn test_parse_bool() {
        let tokens = vec![Token::True, Token::Eof];
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.parse().unwrap(), Value::Bool(true));

        let tokens = vec![Token::False, Token::Eof];
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.parse().unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_parse_numbers() {
        let tokens = vec![Token::Integer(42), Token::Eof];
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.parse().unwrap(), Value::Integer(42));

        let tokens = vec![Token::Float(3.14), Token::Eof];
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.parse().unwrap(), Value::Float(3.14));
    }

    #[test]
    fn test_parse_string() {
        let tokens = vec![Token::String("hello".to_string()), Token::Eof];
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.parse().unwrap(), Value::String("hello".to_string()));
    }

    #[test]
    fn test_parse_empty_array() {
        let tokens = vec![Token::LeftBracket, Token::RightBracket, Token::Eof];
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.parse().unwrap(), Value::Array(vec![]));
    }

    #[test]
    fn test_parse_simple_array() {
        let tokens = vec![
            Token::LeftBracket,
            Token::Integer(1),
            Token::Comma,
            Token::Integer(2),
            Token::Comma,
            Token::Integer(3),
            Token::RightBracket,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let value = parser.parse().unwrap();

        assert_eq!(
            value,
            Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3),
            ])
        );
    }

    #[test]
    fn test_parse_array_with_trailing_comma() {
        let tokens = vec![
            Token::LeftBracket,
            Token::Integer(1),
            Token::Comma,
            Token::Integer(2),
            Token::Comma,
            Token::RightBracket,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let value = parser.parse().unwrap();

        assert_eq!(
            value,
            Value::Array(vec![Value::Integer(1), Value::Integer(2),])
        );
    }

    #[test]
    fn test_parse_empty_object() {
        let tokens = vec![Token::LeftBrace, Token::RightBrace, Token::Eof];
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.parse().unwrap(), Value::Object(HashMap::new()));
    }

    #[test]
    fn test_parse_simple_object() {
        let tokens = vec![
            Token::LeftBrace,
            Token::Identifier("name".to_string()),
            Token::Colon,
            Token::String("Alice".to_string()),
            Token::Comma,
            Token::Identifier("age".to_string()),
            Token::Colon,
            Token::Integer(30),
            Token::RightBrace,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let value = parser.parse().unwrap();

        match value {
            Value::Object(obj) => {
                assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
                assert_eq!(obj.get("age"), Some(&Value::Integer(30)));
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_parse_nested_structure() {
        let tokens = vec![
            Token::LeftBrace,
            Token::Identifier("user".to_string()),
            Token::Colon,
            Token::LeftBrace,
            Token::Identifier("name".to_string()),
            Token::Colon,
            Token::String("Alice".to_string()),
            Token::Comma,
            Token::Identifier("scores".to_string()),
            Token::Colon,
            Token::LeftBracket,
            Token::Integer(95),
            Token::Comma,
            Token::Integer(87),
            Token::Comma,
            Token::Integer(92),
            Token::RightBracket,
            Token::RightBrace,
            Token::RightBrace,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let value = parser.parse().unwrap();

        match value {
            Value::Object(obj) => {
                assert!(obj.contains_key("user"));
                if let Some(Value::Object(user)) = obj.get("user") {
                    assert_eq!(user.get("name"), Some(&Value::String("Alice".to_string())));
                    if let Some(Value::Array(scores)) = user.get("scores") {
                        assert_eq!(scores.len(), 3);
                    } else {
                        panic!("Expected array for scores");
                    }
                } else {
                    panic!("Expected object for user");
                }
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_parse_error_unexpected_token() {
        let tokens = vec![Token::Integer(42), Token::Integer(99), Token::Eof];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("Unexpected tokens"));
    }

    #[test]
    fn test_parse_error_invalid_key() {
        let tokens = vec![
            Token::LeftBrace,
            Token::Integer(42), // Invalid key
            Token::Colon,
            Token::String("value".to_string()),
            Token::RightBrace,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("Expected object key"));
    }

    #[test]
    fn test_parse_object_with_trailing_comma() {
        let tokens = vec![
            Token::LeftBrace,
            Token::Identifier("name".to_string()),
            Token::Colon,
            Token::String("Alice".to_string()),
            Token::Comma,
            Token::RightBrace,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let value = parser.parse().unwrap();

        match value {
            Value::Object(obj) => {
                assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
            }
            _ => panic!("Expected object"),
        }
    }
}

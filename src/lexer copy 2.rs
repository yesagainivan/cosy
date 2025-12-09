use std::{error::Error, fmt};

/// Position information for a token
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }
}

/// A token in the COSY format with position info
#[derive(Debug, Clone, PartialEq)]
pub struct TokenWithPos {
    pub token: Token,
    pub pos: Position,
}

impl TokenWithPos {
    pub fn new(token: Token, pos: Position) -> Self {
        TokenWithPos { token, pos }
    }
}

/// A token in the COSY format
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Identifier(String),
    String(String),
    Integer(i64),
    Float(f64),

    // Keywords
    True,
    False,
    Null,

    // Symbols
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Colon,        // :
    Comma,        // ,

    // End of input
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Identifier(s) => write!(f, "identifier '{}'", s),
            Token::String(s) => write!(f, "string \"{}\"", s),
            Token::Integer(n) => write!(f, "integer {}", n),
            Token::Float(n) => write!(f, "float {}", n),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Null => write!(f, "null"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::Eof => write!(f, "EOF"),
        }
    }
}

/// Lexer error with position information
#[derive(Debug, Clone)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl Error for LexError {}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Lex error at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

/// The COSY lexer
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Create a new lexer from input
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenize the entire input, returning tokens with positions
    pub fn tokenize(&mut self) -> Result<Vec<TokenWithPos>, LexError> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace_and_comments();

            if self.is_at_end() {
                tokens.push(TokenWithPos::new(
                    Token::Eof,
                    Position::new(self.line, self.column),
                ));
                break;
            }

            // Capture position RIGHT before we start lexing the token
            let pos = Position::new(self.line, self.column);
            let token = self.next_token()?;
            tokens.push(TokenWithPos::new(token, pos));
        }

        Ok(tokens)
    }

    // /// Get the next token with its position
    // fn next_token_with_pos(&mut self) -> Result<(Token, Position), LexError> {
    //     let pos = Position::new(self.line, self.column);
    //     let token = self.next_token()?;
    //     Ok((token, pos))
    // }

    /// Get the next token
    fn next_token(&mut self) -> Result<Token, LexError> {
        let ch = self.current_char();

        match ch {
            '{' => {
                self.advance();
                Ok(Token::LeftBrace)
            }
            '}' => {
                self.advance();
                Ok(Token::RightBrace)
            }
            '[' => {
                self.advance();
                Ok(Token::LeftBracket)
            }
            ']' => {
                self.advance();
                Ok(Token::RightBracket)
            }
            ':' => {
                self.advance();
                Ok(Token::Colon)
            }
            ',' => {
                self.advance();
                Ok(Token::Comma)
            }
            '"' => self.lex_string(),
            '-' | '0'..='9' => self.lex_number(),
            'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier(),
            _ => Err(self.error(format!("Unexpected character: '{}'", ch))),
        }
    }

    /// Lex a string literal
    fn lex_string(&mut self) -> Result<Token, LexError> {
        self.advance(); // Skip opening quote
        let mut result = String::new();

        while !self.is_at_end() && self.current_char() != '"' {
            if self.current_char() == '\\' {
                self.advance();
                if self.is_at_end() {
                    return Err(self.error("Unterminated string: unexpected EOF".to_string()));
                }

                let escaped = match self.current_char() {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '"' => '"',
                    _ => {
                        return Err(self.error(format!(
                            "Invalid escape sequence: \\{}",
                            self.current_char()
                        )));
                    }
                };
                result.push(escaped);
                self.advance();
            } else {
                result.push(self.current_char());
                self.advance();
            }
        }

        if self.is_at_end() {
            return Err(self.error("Unterminated string".to_string()));
        }

        self.advance(); // Skip closing quote
        Ok(Token::String(result))
    }

    /// Lex a number (integer or float)
    fn lex_number(&mut self) -> Result<Token, LexError> {
        let start = self.position;
        let start_line = self.line;
        let start_column = self.column;

        // Handle optional minus sign
        if self.current_char() == '-' {
            self.advance();
        }

        // Consume digits
        while !self.is_at_end() && self.current_char().is_ascii_digit() {
            self.advance();
        }

        // Check for float (decimal point or exponent)
        let is_float = if !self.is_at_end() && self.current_char() == '.' {
            // Look ahead to ensure there's a digit after the dot
            if self.position + 1 < self.input.len()
                && self.input[self.position + 1].is_ascii_digit()
            {
                self.advance(); // Consume '.'
                while !self.is_at_end() && self.current_char().is_ascii_digit() {
                    self.advance();
                }
                true
            } else {
                false
            }
        } else {
            false
        };

        // Check for exponent
        let is_float =
            if !self.is_at_end() && (self.current_char() == 'e' || self.current_char() == 'E') {
                self.advance(); // Consume 'e'

                // Optional sign
                if !self.is_at_end() && (self.current_char() == '+' || self.current_char() == '-') {
                    self.advance();
                }

                if self.is_at_end() || !self.current_char().is_ascii_digit() {
                    return Err(LexError {
                        message: "Invalid exponent in number".to_string(),
                        line: start_line,
                        column: start_column,
                    });
                }

                while !self.is_at_end() && self.current_char().is_ascii_digit() {
                    self.advance();
                }
                true
            } else {
                is_float
            };

        let num_str: String = self.input[start..self.position].iter().collect();

        if is_float {
            match num_str.parse::<f64>() {
                Ok(f) => Ok(Token::Float(f)),
                Err(_) => Err(LexError {
                    message: format!("Invalid float: {}", num_str),
                    line: start_line,
                    column: start_column,
                }),
            }
        } else {
            match num_str.parse::<i64>() {
                Ok(i) => Ok(Token::Integer(i)),
                Err(_) => Err(LexError {
                    message: format!("Invalid integer: {}", num_str),
                    line: start_line,
                    column: start_column,
                }),
            }
        }
    }

    /// Lex an identifier or keyword
    fn lex_identifier(&mut self) -> Result<Token, LexError> {
        let start = self.position;

        while !self.is_at_end()
            && (self.current_char().is_alphanumeric() || self.current_char() == '_')
        {
            self.advance();
        }

        let ident: String = self.input[start..self.position].iter().collect();

        let token = match ident.as_str() {
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            _ => Token::Identifier(ident),
        };

        Ok(token)
    }

    /// Skip whitespace and comments
    fn skip_whitespace_and_comments(&mut self) {
        while !self.is_at_end() {
            match self.current_char() {
                ' ' | '\t' | '\r' => self.advance(),
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                }
                '/' if self.peek_next() == Some('/') => {
                    // Skip comment until end of line
                    while !self.is_at_end() && self.current_char() != '\n' {
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    /// Current character
    fn current_char(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.position]
        }
    }

    /// Peek at the next character
    fn peek_next(&self) -> Option<char> {
        if self.position + 1 < self.input.len() {
            Some(self.input[self.position + 1])
        } else {
            None
        }
    }

    /// Move to the next character
    fn advance(&mut self) {
        if !self.is_at_end() {
            if self.input[self.position] == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }

    /// Check if we're at the end of input
    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    /// Create an error with current position
    fn error(&self, message: String) -> LexError {
        LexError {
            message,
            line: self.line,
            column: self.column,
        }
    }
}

// In lexer.rs, fix the test:

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        let mut lexer = Lexer::new("{ } [ ] : ,");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 7); // 6 tokens + EOF
        assert_eq!(tokens[0].token, Token::LeftBrace);
        assert_eq!(tokens[1].token, Token::RightBrace);
    }

    #[test]
    fn test_position_tracking() {
        // Test with newline between tokens
        let mut lexer = Lexer::new("true\nfalse");
        let tokens = lexer.tokenize().unwrap();

        // Debug output
        println!(
            "Tokens: {:?}",
            tokens.iter().map(|t| (&t.token, t.pos)).collect::<Vec<_>>()
        );

        assert_eq!(tokens[0].pos, Position::new(1, 1)); // true on line 1, col 1
        assert_eq!(tokens[1].pos, Position::new(2, 1)); // false on line 2, col 1
    }

    #[test]
    fn test_complex_positions() {
        let input = r#"
name: "Alice"
age: 30
"#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        // Verify some positions exist
        assert!(tokens.iter().any(|t| t.pos.line > 1));
    }

    #[test]
    fn test_position_tracking_multiline() {
        // More explicit test
        let input = "a\nb\nc";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        // a should be at line 1
        assert_eq!(tokens[0].pos.line, 1);
        // b should be at line 2
        assert_eq!(tokens[1].pos.line, 2);
        // c should be at line 3
        assert_eq!(tokens[2].pos.line, 3);
    }
}

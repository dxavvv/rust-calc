// ===========================================
// LEXICAL ANALYZER MODULE
// ===========================================
// Converts source string into token stream
// ===========================================

use super::token::Token;

pub struct Lexer {
    input: Vec<char>,    // Source code as character vector
    position: usize,     // Current reading position
}

impl Lexer {
    /// Creates new lexer for given input string
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    /// Reads and returns next token from input
    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        
        match self.current_char() {
            Some(c) => match c {
                // Single-character tokens
                '(' => self.consume(Token::LeftParenthesis),
                ')' => self.consume(Token::RightParenthesis),
                '+' => self.consume(Token::Plus),
                '-' => self.consume(Token::Minus),
                '*' => self.consume(Token::Multiply),
                '/' => self.consume(Token::Divide),
                '^' => self.consume(Token::Caret),
                '=' => self.consume(Token::Equals),
                
                // Complex tokens requiring multiple characters
                _ if c.is_ascii_digit() => self.read_number(),
                _ if c.is_ascii_alphabetic() => self.read_symbol(),
                
                // Invalid character handling
                _ => Err(format!("Unexpected character: '{}'", c)),
            },
            None => Ok(Token::EndOfFile), // End of input
        }
    }

    /// Reads numeric literal from input
    fn read_number(&mut self) -> Result<Token, String> {
        let start = self.position;
        
        // Consume all consecutive digits and decimal points
        while let Some(c) = self.current_char() {
            if !c.is_ascii_digit() && c != '.' {
                break;
            }
            self.advance();
        }
        
        // Parse collected characters into f64
        let number_str: String = self.input[start..self.position].iter().collect();
        number_str
            .parse()
            .map(Token::Number)
            .map_err(|_| format!("Invalid number format: '{}'", number_str))
    }

    /// Reads symbol/identifier from input
    fn read_symbol(&mut self) -> Result<Token, String> {
        let start = self.position;
        
        // Consume all consecutive alphabetic characters
        while let Some(c) = self.current_char() {
            if !c.is_ascii_alphabetic() {
                break;
            }
            self.advance();
        }
        
        let symbol: String = self.input[start..self.position].iter().collect();
        Ok(Token::Symbol(symbol))
    }

    // ================ HELPER METHODS ================
    
    /// Returns current character without consuming it
    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    /// Advances to next character in input
    fn advance(&mut self) {
        self.position += 1;
    }

    /// Consumes current character and returns given token
    fn consume(&mut self, token: Token) -> Result<Token, String> {
        self.advance();
        Ok(token)
    }

    /// Skips whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }
}

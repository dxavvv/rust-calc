// ===========================================
// TOKEN DEFINITIONS MODULE
// ===========================================
// Defines lexical tokens and their properties
// ===========================================

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LeftParenthesis,    // (
    RightParenthesis,   // )
    Plus,               // +
    Minus,              // -
    Multiply,           // *
    Divide,             // /
    Caret,              // ^
    Equals,             // =
    Number(f64),        // Numeric literal
    Symbol(String),     // Identifier/function name
    EndOfFile,          // End of input marker
}

/// Defines operator associativity for parsing
#[derive(Debug, PartialEq, Clone)]
pub enum Associativity {
    Left,   // Left-associative: a + b + c = (a + b) + c
    Right,  // Right-associative: a ^ b ^ c = a ^ (b ^ c)
}

impl Token {
    /// Returns precedence and associativity for operator tokens
    /// Higher precedence = tighter binding
    pub fn precedence_and_associativity(&self) -> Option<(u8, Associativity)> {
        match self {
            Token::Plus | Token::Minus => Some((1, Associativity::Left)),     // Lowest precedence
            Token::Multiply | Token::Divide => Some((2, Associativity::Left)), // Medium precedence
            Token::Caret => Some((3, Associativity::Right)),                  // Highest precedence
            _ => None, // Non-operator tokens return None
        }
    }

    /// Checks if token marks end of input
    pub fn is_eof(&self) -> bool {
        matches!(self, Token::EndOfFile)
    }
}

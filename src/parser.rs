// ===========================================
// SYNTAX ANALYZER MODULE
// ===========================================
// Converts token stream into Abstract Syntax Tree
// ===========================================

use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::{Associativity, Token};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    next_token: Option<Token>, // One-token lookahead
}

impl Parser {
    /// Creates new parser with initialized token stream
    pub fn new(input: &str) -> Result<Self, String> {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token()?;
        
        Ok(Self {
            lexer,
            current_token,
            next_token: None,
        })
    }

    /// Parses entire expression and returns AST root
    pub fn parse(&mut self) -> Result<Box<dyn AstNode>, String> {
        let ast = self.parse_expression(0)?; // Start with minimum precedence
        self.expect_end()?; // Ensure no extra tokens remain
        Ok(ast)
    }

    /// Recursive descent parser for expressions using precedence climbing
    fn parse_expression(&mut self, min_precedence: u8) -> Result<Box<dyn AstNode>, String> {
        // Parse left-hand side atom (number, variable, function call, etc.)
        let mut left_expr = self.parse_atom()?;
        
        // Process operators with sufficient precedence
        while let Some((precedence, associativity)) = self.current_token.precedence_and_associativity() {
            if precedence < min_precedence {
                break;
            }
            
            let operator = self.current_token.clone();
            self.advance()?; // Consume operator
            
            // Parse right-hand side with appropriate precedence
            let next_min_precedence = match associativity {
                Associativity::Left => precedence + 1,
                Associativity::Right => precedence,
            };
            
            let right_expr = self.parse_expression(next_min_precedence)?;
            left_expr = self.create_binary_node(operator, left_expr, right_expr);
        }
        
        Ok(left_expr)
    }

    /// Parses atomic expressions (leaf nodes or function calls)
    fn parse_atom(&mut self) -> Result<Box<dyn AstNode>, String> {
        match &self.current_token {
            Token::Number(value) => {
                let node = Box::new(NumberNode { value: *value });
                self.advance()?;
                Ok(node)
            }
            
            Token::Symbol(name) => {
                let symbol_name = name.clone();
                self.advance()?;
                
                match self.current_token {
                    Token::LeftParenthesis => self.parse_function_call(symbol_name),
                    Token::Equals => self.parse_assignment(symbol_name),
                    _ => Ok(Box::new(VariableNode { name: symbol_name })),
                }
            }
            
            Token::LeftParenthesis => {
                self.advance()?; // Consume '('
                let expr = self.parse_expression(0)?; // Parse inner expression
                self.expect_token(Token::RightParenthesis)?; // Expect ')'
                Ok(expr)
            }
            
            _ => Err(format!("Unexpected token: {:?}", self.current_token)),
        }
    }

    /// Parses function calls (sin, cos, sqrt, print)
    fn parse_function_call(&mut self, function_name: String) -> Result<Box<dyn AstNode>, String> {
        self.expect_token(Token::LeftParenthesis)?;
        let argument = self.parse_expression(0)?;
        self.expect_token(Token::RightParenthesis)?;
        
        match function_name.as_str() {
            "sin" => Ok(Box::new(SineNode { argument })),
            "cos" => Ok(Box::new(CosineNode { argument })),
            "sqrt" => Ok(Box::new(SquareRootNode { argument })),
            "print" => Ok(Box::new(PrintNode { argument })),
            _ => Err(format!("Unknown function: {}", function_name)),
        }
    }

    /// Parses variable assignments (let x = ...)
    fn parse_assignment(&mut self, variable_name: String) -> Result<Box<dyn AstNode>, String> {
        self.expect_token(Token::Equals)?;
        let value_expr = self.parse_expression(0)?;
        Ok(Box::new(AssignmentNode {
            variable_name,
            value: value_expr,
        }))
    }

    // ================ HELPER METHODS ================
    
    /// Creates binary operation node from operator token
    fn create_binary_node(
        &self,
        operator: Token,
        left: Box<dyn AstNode>,
        right: Box<dyn AstNode>,
    ) -> Box<dyn AstNode> {
        match operator {
            Token::Plus => Box::new(AddNode { left, right }),
            Token::Minus => Box::new(SubtractNode { left, right }),
            Token::Multiply => Box::new(MultiplyNode { left, right }),
            Token::Divide => Box::new(DivideNode { left, right }),
            Token::Caret => Box::new(PowerNode { base: left, exponent: right }),
            _ => panic!("Unsupported binary operator"),
        }
    }

    /// Advances to next token in stream
    fn advance(&mut self) -> Result<(), String> {
        self.current_token = match self.next_token.take() {
            Some(token) => token,
            None => self.lexer.next_token()?,
        };
        Ok(())
    }

    /// Verifies current token matches expected, then advances
    fn expect_token(&mut self, expected: Token) -> Result<(), String> {
        if self.current_token == expected {
            self.advance()
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, self.current_token))
        }
    }

    /// Ensures input is fully consumed
    fn expect_end(&mut self) -> Result<(), String> {
        if self.current_token.is_eof() {
            Ok(())
        } else {
            Err(format!("Unexpected token at end: {:?}", self.current_token))
        }
    }
}

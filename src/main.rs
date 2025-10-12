// ===========================================
// CALCULATOR REPL - MAIN ENTRY POINT
// ===========================================
// Interactive Read-Eval-Print Loop for mathematical expressions
// ===========================================

use std::collections::HashMap;
use std::io::{self, Write};

mod ast;
mod lexer;
mod parser;
mod token;

use crate::parser::Parser;

/// Evaluates mathematical expression and returns result
fn evaluate_expression(input: &str, env: &mut HashMap<String, f64>) -> Result<f64, String> {
    let mut parser = Parser::new(input)?;
    let ast = parser.parse()?;
    ast.evaluate(env).ok_or_else(|| "Evaluation failed".to_string())
}

fn main() {
    println!("====================================");
    println!("     RUST CALCULATOR REPL");
    println!("====================================");
    println!("Supported operations: + - * / ^");
    println!("Functions: sin(x), cos(x), sqrt(x), print(x)");
    println!("Variables: let x = 5, then use x in expressions");
    println!("Type 'quit' to exit");
    println!("====================================\n");

    // Initialize environment with mathematical constants
    let mut environment = HashMap::new();
    environment.insert("pi".to_string(), std::f64::consts::PI);
    environment.insert("e".to_string(), std::f64::consts::E);

    // REPL loop
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) | Err(_) => break, // Exit on EOF or error
            Ok(_) => {
                let input = input.trim();
                
                // Exit condition
                if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
                    println!("Goodbye!");
                    break;
                }
                
                // Skip empty inputs
                if input.is_empty() {
                    continue;
                }

                // Evaluate expression and handle result
                match evaluate_expression(input, &mut environment) {
                    Ok(result) => {
                        // Result already printed if using print() function
                        if !input.contains("print") {
                            println!("=> {}", result);
                        }
                    }
                    Err(error) => {
                        eprintln!("Error: {}", error);
                    }
                }
            }
        }
    }
}

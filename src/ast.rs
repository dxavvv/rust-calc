// ===========================================
// ABSTRACT SYNTAX TREE MODULE
// ===========================================
// Defines the AST structure and evaluation logic
// ===========================================

use std::collections::HashMap;

/// Core trait for all AST nodes - enables polymorphic evaluation
pub trait AstNode {
    /// Evaluates the node and returns optional f64 result
    fn evaluate(&self, env: &mut HashMap<String, f64>) -> Option<f64>;
}

// ==================== NUMERIC NODE ====================
/// Represents a numeric literal (e.g., 42, 3.14)
pub struct NumberNode {
    pub value: f64,
}

impl AstNode for NumberNode {
    fn evaluate(&self, _env: &mut HashMap<String, f64>) -> Option<f64> {
        Some(self.value) // Always returns the stored numeric value
    }
}

// ==================== BINARY OPERATIONS ====================
/// Macro to generate binary operation nodes (reduces code duplication)
macro_rules! binary_operation {
    ($struct_name:ident, $operator:tt) => {
        /// Binary operation node with left and right children
        pub struct $struct_name {
            pub left: Box<dyn AstNode>,
            pub right: Box<dyn AstNode>,
        }

        impl AstNode for $struct_name {
            fn evaluate(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
                // Use ? operator for automatic error propagation
                Some(self.left.evaluate(env)? $operator self.right.evaluate(env)?)
            }
        }
    };
}

// Generate common binary operation nodes
binary_operation!(AddNode, +);
binary_operation!(SubtractNode, -);
binary_operation!(MultiplyNode, *);
binary_operation!(DivideNode, /);

// ==================== POWER OPERATION ====================
/// Exponentiation node (base ^ exponent)
pub struct PowerNode {
    pub base: Box<dyn AstNode>,
    pub exponent: Box<dyn AstNode>,
}

impl AstNode for PowerNode {
    fn evaluate(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        Some(self.base.evaluate(env)?.powf(self.exponent.evaluate(env)?))
    }
}

// ==================== UNARY FUNCTIONS ====================
/// Macro to generate unary mathematical function nodes
macro_rules! unary_function {
    ($struct_name:ident, $function:expr) => {
        /// Unary function node with single argument
        pub struct $struct_name {
            pub argument: Box<dyn AstNode>,
        }

        impl AstNode for $struct_name {
            fn evaluate(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
                Some($function(self.argument.evaluate(env)?))
            }
        }
    };
}

// Generate mathematical function nodes
unary_function!(SineNode, f64::sin);
unary_function!(CosineNode, f64::cos);
unary_function!(SquareRootNode, f64::sqrt);

// ==================== OUTPUT OPERATION ====================
/// Node that prints result to console while evaluating
pub struct PrintNode {
    pub argument: Box<dyn AstNode>,
}

impl AstNode for PrintNode {
    fn evaluate(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        let result = self.argument.evaluate(env)?;
        println!("=> {}", result); // Display result with prompt-like format
        Some(result)
    }
}

// ==================== VARIABLE OPERATIONS ====================
/// Node representing variable reference (e.g., x, pi)
pub struct VariableNode {
    pub name: String,
}

impl AstNode for VariableNode {
    fn evaluate(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        env.get(&self.name).copied() // Lookup variable in environment
    }
}

/// Node for variable assignment (e.g., let x = 5)
pub struct AssignmentNode {
    pub variable_name: String,
    pub value: Box<dyn AstNode>,
}

impl AstNode for AssignmentNode {
    fn evaluate(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        let computed_value = self.value.evaluate(env)?;
        env.insert(self.variable_name.clone(), computed_value);
        Some(computed_value)
    }
}

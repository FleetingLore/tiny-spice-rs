//! Expression data type used by parameters.
//!
//! Parsing functions live in `spice-parser::value`.

use std::fmt;

/// A parameter expression: either a literal value or a named identifier.
#[derive(Clone, Debug)]
pub enum Expression {
    Literal(f64),
    Identifier(String),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Literal(ref p) => write!(f, "[{}]", p),
            Expression::Identifier(ref p) => write!(f, "[Identifier {}]", p),
        }
    }
}

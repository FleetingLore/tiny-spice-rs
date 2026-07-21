//! Resistor Implementation

use std::fmt;

use crate::circuit::NodeId;

/// Resistor Implementation
#[allow(dead_code)]
#[derive(Clone)]
pub struct Resistor {
    pub ident: String,
    pub a: NodeId,
    pub b: NodeId,
    pub value: f64, // Ohms
}

impl fmt::Display for Resistor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "R a:{} b:{} {} Ohms ({})",
            self.a, self.b, self.value, self.ident
        )
    }
}

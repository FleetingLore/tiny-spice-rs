//! Independent Source Implementations

use std::fmt;

use crate::circuit::NodeId;

/// Current Source Implementation
#[allow(dead_code)]
#[derive(Clone)]
pub struct CurrentSource {
    pub p: NodeId,
    pub n: NodeId,
    pub value: f64, // Amperes
}

impl fmt::Display for CurrentSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "I p:{} n:{} {} A", self.p, self.n, self.value)
    }
}

/// Voltage Source Implementation
#[allow(dead_code)]
#[derive(Clone)]
pub struct VoltageSource {
    pub p: NodeId,
    pub n: NodeId,
    pub value: f64, // Volts
    pub idx: usize, // index of voltage source in "known" column
}

impl fmt::Display for VoltageSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "V a:{} b:{} {} Volts", self.p, self.n, self.value)
    }
}

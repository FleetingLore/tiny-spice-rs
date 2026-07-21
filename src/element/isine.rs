//! Sinusoidal Current Source Implementation

use std::fmt;

use crate::circuit::NodeId;
use std::f64::consts::PI;

#[derive(Clone)]
pub struct CurrentSourceSine {
    pub p: NodeId,
    pub n: NodeId,
    pub vo: f64,   // offset (A)
    pub va: f64,   // amplitude (A)
    pub freq: f64, // frequency (HZ)
}

impl CurrentSourceSine {
    // calculate the value at a certain time
    pub fn evaluate(&self, t: f64) -> f64 {
        self.vo + self.va * (2.0 * PI * self.freq * t).sin()
    }
}

impl fmt::Display for CurrentSourceSine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Isin p:{} n:{} = {} + {} * sin(2pi {})",
            self.p, self.n, self.vo, self.va, self.freq
        )
    }
}

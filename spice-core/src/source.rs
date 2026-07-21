//! Circuit source abstraction.
//!
//! Implementations:
//! - [`spice::Reader`] – standard SPICE netlist parser

use std::path::Path;

use crate::analysis::Configuration;
use crate::circuit::Circuit;

/// Trait for loading a circuit from some source.
pub trait CircuitSource {
    /// Read and parse the source. Returns `true` if errors occurred.
    fn read(&mut self, path: &Path) -> bool;

    /// Return the expanded (flat) circuit.
    fn get_expanded_circuit(&self) -> Circuit;

    /// Return the simulation configuration parsed from the source.
    fn configuration(&self) -> &Configuration;
}

//! Tiny-Spice-RS - A teeny weeny SPICE circuit simulator

/// Trace macro for development debugging.
/// Enabled via `cargo build --features dev`.
macro_rules! trace {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        if cfg!(feature = "dev") {
            println!(concat!("[{}] ", $fmt), module_path!(), $($($arg)*)?);
        }
    };
}

// Circuit and Analysis Datastructures
pub mod analysis;
pub mod bracket_expression;
pub mod circuit;
pub mod parameter;

// Simulation Engine
pub mod engine;

// Device Models
pub mod element;

// Waveform dumper
pub mod wavewriter;

// Read and elaborate SPICE circuit descriptions
pub mod expander;
pub mod spice;

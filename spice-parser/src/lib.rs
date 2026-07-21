//! SPICE netlist parser.
//!
//! Implements [`tiny_spice::source::CircuitSource`] for standard SPICE files.

/// Trace macro for development debugging.
/// Enabled via `cargo build --features dev` on `tiny_spice`.
macro_rules! trace {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        if cfg!(feature = "dev") {
            println!(concat!("[spice-parser] ", $fmt), $($($arg)*)?);
        }
    };
}

pub mod spice;
pub mod value;

pub use spice::Reader;

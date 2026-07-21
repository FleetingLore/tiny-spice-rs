//! Simulation progress reporting abstraction.
//!
//! Implementations:
//! - [`TerminalReporter`] – plain stdout/stderr (default, zero deps)
//! - `spice_tui::TuiReporter` – progress bars via indicatif

/// Reporter trait for simulation progress and diagnostics.
pub trait Reporter {
    /// Print banner at simulation start.
    fn banner(&self);

    /// Informational message (stdout).
    fn info(&self, msg: &str);

    /// Warning message (stderr).
    fn warn(&self, msg: &str);

    /// Error message (stderr).
    fn error(&self, msg: &str);

    /// Called when transient analysis begins.
    fn begin_transient(&self, tstop: f64);

    /// Called each timestep during transient analysis.
    fn tick_transient(&self, t_now: f64, t_stop: f64);

    /// Called when transient analysis ends.
    fn finish_transient(&self);
}

// ---------------------------------------------------------------------------
// Default: plain terminal output
// ---------------------------------------------------------------------------

/// Plain terminal reporter using `println!` / `eprintln!`.
pub struct TerminalReporter;

impl Reporter for TerminalReporter {
    fn banner(&self) {
        println!("**********************************************");
        println!("***          Tiny-SPICE-Simulator          ***");
        println!("***       (c) CrapCadCorp 2017-2023        ***");
        println!("*** No Patents Pending, No rights reserved ***");
        println!("**********************************************");
    }

    fn info(&self, msg: &str) {
        println!("*INFO* {msg}");
    }

    fn warn(&self, msg: &str) {
        eprintln!("*WARN* {msg}");
    }

    fn error(&self, msg: &str) {
        eprintln!("*ERROR* {msg}");
    }

    fn begin_transient(&self, _tstop: f64) {}

    fn tick_transient(&self, _t_now: f64, _t_stop: f64) {}

    fn finish_transient(&self) {}
}

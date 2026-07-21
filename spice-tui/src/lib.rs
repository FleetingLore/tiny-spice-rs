//! TUI reporter using indicatif progress bars

use std::cell::RefCell;

use indicatif::{ProgressBar, ProgressStyle};
use tiny_spice::report::Reporter;

pub struct TuiReporter {
    bar: RefCell<Option<ProgressBar>>,
}

impl TuiReporter {
    pub fn new() -> Self {
        Self {
            bar: RefCell::new(None),
        }
    }
}

impl Default for TuiReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for TuiReporter {
    fn banner(&self) {}

    fn info(&self, msg: &str) {
        if let Some(ref bar) = *self.bar.borrow() {
            bar.println(format!("*INFO* {msg}"));
        } else {
            println!("*INFO* {msg}");
        }
    }

    fn warn(&self, msg: &str) {
        if let Some(ref bar) = *self.bar.borrow() {
            bar.println(format!("*WARN* {msg}"));
        } else {
            eprintln!("*WARN* {msg}");
        }
    }

    fn error(&self, msg: &str) {
        if let Some(ref bar) = *self.bar.borrow() {
            bar.println(format!("*ERROR* {msg}"));
        } else {
            eprintln!("*ERROR* {msg}");
        }
    }

    fn begin_transient(&self, tstop: f64) {
        let bar = ProgressBar::new(tstop as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% {msg}",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        bar.set_message("transient analysis...");
        *self.bar.borrow_mut() = Some(bar);
    }

    fn tick_transient(&self, t_now: f64, _t_stop: f64) {
        if let Some(ref bar) = *self.bar.borrow() {
            bar.set_position(t_now as u64);
        }
    }

    fn finish_transient(&self) {
        if let Some(ref bar) = self.bar.borrow_mut().take() {
            bar.finish_with_message("transient complete");
        }
    }
}

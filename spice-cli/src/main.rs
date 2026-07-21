//! Tiny-SPICE CLI entry point

use std::env;
use std::path::Path;

use spice_parser::Reader;
use spice_tui::TuiReporter;
use tiny_spice::engine::Engine;
use tiny_spice::source::CircuitSource;

/// Read a spice file, and execute it
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("*FATAL* Please supply a SPICE filename");
        return;
    }

    let spice_file = Path::new(&args[1]);
    if !spice_file.exists() {
        eprintln!("*FATAL* Can't access spice file: '{}'", args[1]);
        return;
    }

    // Choose circuit source based on file extension
    let mut source: Box<dyn CircuitSource> = match spice_file.extension().and_then(|e| e.to_str()) {
        Some("spi") | Some("cir") | Some("sp") => Box::new(Reader::new()),
        other => {
            eprintln!(
                "*FATAL* Unsupported file format: {:?}",
                other.unwrap_or("none")
            );
            return;
        }
    };

    let errors_exist = source.read(spice_file);
    if errors_exist {
        eprintln!("*FATAL* Errors in SPICE Deck so not doing simulations");
        return;
    }

    let ckt = source.get_expanded_circuit();
    ckt.list_elements();
    ckt.list_nodes();
    let cfg = source.configuration();

    let mut eng = Engine::new();
    eng.set_reporter(Box::new(TuiReporter::new()));

    if let Some(_stats) = eng.go(&ckt, cfg) {
        println!("\n*INFO* Done");
    } else {
        eprintln!("\n*ERROR* analysis type not set or unsupported");
    }
}

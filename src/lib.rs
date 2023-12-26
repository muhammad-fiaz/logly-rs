// lib.rs

// Import the logly module
mod logly;

use log::{Level};

// Use the Rustly logger from the logly module
use logly::Rustly;

fn main() {
    // Create an instance of Rustly with desired configurations
    let mut rustly = Rustly::new(true, true);

    // Start logging
    rustly.start_logging();

    // Log messages with different levels
    rustly.log(Level::Info, "This is an information message.");
    rustly.log(Level::Error, "This is an error message.");
    rustly.log(Level::Trace, "This is a trace message.");

    // Stop logging (optional)
    rustly.stop_logging();
}

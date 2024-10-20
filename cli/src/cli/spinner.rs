//! This module provides functionality for displaying a simple spinner in the terminal,
//! which can be useful for indicating ongoing processing or loading states. It allows
//! starting and stopping the spinner, ensuring that it runs in a separate thread to
//! maintain responsiveness in the main application flow.
//!
//! ## Functions
//!
//! - [`start_spinner`] - Starts the spinner in a separate thread, indicating progress or loading state.
//! - [`stop_spinner`] - Stops the spinner and clears the last displayed character in the terminal.
//!
//! ## Examples
//!
//! ```
//! use crate::cli::spinner::{start_spinner, stop_spinner};
//!
//! let spinner = start_spinner();
//! // Simulate some loading work
//! thread::sleep(Duration::from_secs(5));
//! stop_spinner(spinner);
//! ```
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time::Duration};

pub fn start_spinner() -> Arc<AtomicBool> {
    let spinning = Arc::new(AtomicBool::new(true));
    let spin = spinning.clone();

    thread::spawn(move || {
        let spinner_chars = ['|', '/', '-', '\\'];
        let mut index = 0;
        while spin.load(Ordering::Relaxed) {
            print!("\r{}", spinner_chars[index]);
            index = (index + 1) % spinner_chars.len();
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    spinning
}

pub fn stop_spinner(spinning: Arc<AtomicBool>) {
    spinning.store(false, Ordering::Relaxed);
    // Clear the last spinner character
    print!("\r \r");
    std::io::stdout().flush().unwrap();
}

use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time::Duration};

// Function to show a spinner
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

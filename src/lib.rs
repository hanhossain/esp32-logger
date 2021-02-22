//! Logger for the ESP32
//!
//! # Example
//! ```
//! use log::info;
//!
//! fn main() -> ! {
//!     esp32_logger::init();
//!
//!     let mut counter = 0;
//!
//!     loop {
//!         info!("counter: {}", counter);
//!         counter += 1;
//!     }
//! }
//!
//! ```
#![no_std]

use esp32_hal::dprintln;
use log::{Level, LevelFilter, Metadata, Record};

static LOGGER: Logger = Logger;

/// Setup the logger with the default (info) level.
pub fn init() {
    init_with_level(LevelFilter::Info);
}

/// Setup the logger with a specific level.
pub fn init_with_level(level: LevelFilter) {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(level);

    log::trace!("Initialized logger. Level = {}.", level);
}

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            dprintln!("[ {} ] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

//! Logger module
//! A simple logger implementation use log crate

use log::{Level, LevelFilter, Log, Metadata, Record};

struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let color = match record.level() {
            Level::Error => "\x1b[31m", // red
            Level::Warn => "\x1b[93m",  // yellow
            Level::Info => "\x1b[34m",  // blue
            Level::Debug => "\x1b[31m", // brown
            Level::Trace => "\x1b[37m", // white
        };

        println!(
            "{}[{:^5}]: {}\x1b[0m",
            color,
            record.level(),
            record.args(),
        );
        
    }

    fn flush(&self) {}
}

pub fn init() {
    static LOGGER: Logger = Logger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    })
}
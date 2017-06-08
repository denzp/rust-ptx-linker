use std::io::stderr;
use std::fmt::Arguments;

use fern::{Dispatch, FormatCallback};
use log::{LogLevel, LogLevelFilter, LogRecord};
use colored::*;

pub fn setup_logging() {
    Dispatch::new()
        .format(logging_handler)
        .level(LogLevelFilter::Debug)
        .chain(stderr())
        .apply()
        .unwrap();
}

fn logging_handler(out: FormatCallback, message: &Arguments, record: &LogRecord) {
    let level = match record.level() {
        LogLevel::Trace => format!("{}{}{}", "[".bold(), "TRACE".cyan(), "]".bold()),
        LogLevel::Debug => format!("{}{}{}", "[".bold(), "DEBUG".blue(), "]".bold()),
        LogLevel::Info => format!(" {}{}{}", "[".bold(), "INFO".green().bold(), "]".bold()),
        LogLevel::Warn => format!(" {}{}{}", "[".bold(), "WARN".yellow().bold(), "]".bold()),
        LogLevel::Error => format!("{}{}{}", "[".bold(), "ERROR".red().bold(), "]".bold()),
    };

    out.finish(format_args!("{} {}", level, message));
}


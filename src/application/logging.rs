use std::io::stderr;
use std::fmt::Arguments;

use fern::{Dispatch, FormatCallback};
use log::{LogLevel, LogLevelFilter, LogRecord};
use colored::*;

pub trait AlignedOutputString: ToString {
    fn prefix_with_spaces(&self, spaces_count: usize) -> String {
        let separator = String::from("\n") + &" ".repeat(spaces_count);

        self.to_string()
            .split('\n')
            .collect::<Vec<_>>()
            .join(&separator)
    }
}

impl AlignedOutputString for String {}

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

    let message = format!("{}", message);

    out.finish(format_args!("{} {}", level, message.prefix_with_spaces(8)));
}


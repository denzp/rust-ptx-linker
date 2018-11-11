use std::fmt::Arguments;
use std::io::stderr;

use colored::*;
use fern::{Dispatch, FormatCallback};
use log::{Level, LevelFilter, Record};

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
        .level(LevelFilter::Info)
        .chain(stderr())
        .apply()
        .unwrap();
}

fn logging_handler(out: FormatCallback, message: &Arguments, record: &Record) {
    let level = match record.level() {
        Level::Trace => format!("{}{}{}", "[".bold(), "TRACE".cyan(), "]".bold()),
        Level::Debug => format!("{}{}{}", "[".bold(), "DEBUG".blue(), "]".bold()),
        Level::Info => format!(" {}{}{}", "[".bold(), "INFO".green().bold(), "]".bold()),
        Level::Warn => format!(" {}{}{}", "[".bold(), "WARN".yellow().bold(), "]".bold()),
        Level::Error => format!("{}{}{}", "[".bold(), "ERROR".red().bold(), "]".bold()),
    };

    let message = format!("{}", message);

    out.finish(format_args!("{} {}", level, message.prefix_with_spaces(8)));
}

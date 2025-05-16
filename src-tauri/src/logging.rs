// logging utils

use colored::Colorize;
use std::{fmt::Debug, time::SystemTime};
use chrono;

#[allow(dead_code)]
pub enum LoggingLevel {
    Trace,
    Info,
    Warn,
    Error,
    Fatal,
}

#[allow(unreachable_patterns)]
pub fn logging_function(lvl: LoggingLevel, str: &str) {
    let time = chrono::offset::Local::now().to_string();
    let logging_level = match lvl {
        LoggingLevel::Trace => "TRACE".purple(),
        LoggingLevel::Info => "INFO ".blue(),
        LoggingLevel::Warn => "WARN ".yellow(),
        LoggingLevel::Error => "ERROR".red(),
        LoggingLevel::Fatal => "FATAL".black().on_bright_red(),
        _ => "not yet implemented".white()
    };
    println!( "@ [{}] {} | {}", time, logging_level.to_string(), str);
}

#[macro_export]
macro_rules! trace {
    ( $x: expr ) => { logging_function(LoggingLevel::Trace, $x); };
}

#[macro_export]
macro_rules! info {
    ( $x: expr ) => { logging_function(LoggingLevel::Info,  $x); };
}

#[macro_export]
macro_rules! warn {
    ( $x: expr ) => { logging_function(LoggingLevel::Warn,  $x); };
}

#[macro_export]
macro_rules! error {
    ( $x: expr ) => { logging_function(LoggingLevel::Error, $x); };
}

#[macro_export]
macro_rules! fatal {
    ( $x: expr ) => { logging_function(LoggingLevel::Fatal, $x); };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_length() {
        logging_function(LoggingLevel::Trace, "wow");
        logging_function(LoggingLevel::Info , "wow");
        logging_function(LoggingLevel::Warn , "wow");
        logging_function(LoggingLevel::Error, "wow");
        logging_function(LoggingLevel::Fatal, "wow");

        trace!("");
        info!("");
        warn!("");
        error!("");
        fatal!("");        
    }
}

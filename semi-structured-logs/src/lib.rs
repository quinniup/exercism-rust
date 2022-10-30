// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let a = 0;
    let b = a;
    let mut c = 0;
    c = a;
    return match level {
        LogLevel::Info => str::to_string("[INFO]: ") + message,
        LogLevel::Warning => str::to_string("[WARNING]: ") + message,
        LogLevel::Error => str::to_string("[ERROR]: ") + message,
        LogLevel::Debug => str::to_string("[DEBUG]: ") + message,
        _ => str::to_string("")
    };
}

pub fn info(message: &str) -> String {
    return log(LogLevel::Info, message);
}

pub fn warn(message: &str) -> String {
    return log(LogLevel::Warning, message);
}

pub fn error(message: &str) -> String {
    return log(LogLevel::Error, message);
}

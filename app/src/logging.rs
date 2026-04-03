use crate::cli::LogLevel;
use tracing_subscriber::{EnvFilter, fmt};

pub fn init(quiet: bool, verbose: u8, log_level: Option<LogLevel>) {
    let level = if quiet {
        tracing::Level::ERROR
    } else if let Some(level) = log_level {
        match level {
            LogLevel::Error => tracing::Level::ERROR,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Trace => tracing::Level::TRACE,
        }
    } else {
        match verbose {
            0 => tracing::Level::WARN,
            1 => tracing::Level::INFO,
            2 => tracing::Level::DEBUG,
            _ => tracing::Level::TRACE,
        }
    };

    let filter = EnvFilter::from_default_env().add_directive(level.into());
    let _ = fmt().with_env_filter(filter).with_target(false).try_init();
}

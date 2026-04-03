use crate::cli::{Cli, LogLevel};
use crate::profile::Profile;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub profile_path: Option<PathBuf>,
    pub quiet: bool,
    pub verbose: u8,
    pub log_level: Option<LogLevel>,
}

impl RuntimeConfig {
    pub fn from_cli(cli: &Cli, profile: Profile) -> Self {
        let quiet = cli.runtime.quiet || profile.quiet.unwrap_or(false);
        let verbose = cli.runtime.verbose.max(profile.verbose.unwrap_or(0));
        let log_level = cli
            .runtime
            .log_level
            .or_else(|| profile.log_level.as_deref().and_then(parse_log_level));

        Self {
            profile_path: cli.runtime.profile.clone(),
            quiet,
            verbose,
            log_level,
        }
    }
}

pub fn parse_log_level(value: &str) -> Option<LogLevel> {
    match value.to_ascii_lowercase().as_str() {
        "error" => Some(LogLevel::Error),
        "warn" => Some(LogLevel::Warn),
        "info" => Some(LogLevel::Info),
        "debug" => Some(LogLevel::Debug),
        "trace" => Some(LogLevel::Trace),
        _ => None,
    }
}

use clap::ValueEnum;

/// Log level enumeration for command-line argument parsing.
#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    /// Convert the log level enum variant to its string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }
    }
}

/// Initialize the logging system with the specified log level.
///
/// The log level can be set via the `RUST_LOG` environment variable.
/// If the environment variable is not set, the provided `log_level` parameter
/// will be used as the default.
///
/// # Arguments
///
/// * `log_level` - The default log level to use if `RUST_LOG` is not set.
///   Valid values: "trace", "debug", "info", "warn", "error"
pub fn init_logging(log_level: &str) {
    // First try to use the environment variable if set, otherwise use the CLI argument
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level));

    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}

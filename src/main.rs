mod db;
mod models;
mod services;
mod ui;
mod utils;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "rev-report")]
#[command(about = "Revenue Report CSV Data Loader", long_about = None)]
struct Args {
    /// Set the logging level (trace, debug, info, warn, error)
    #[arg(
        short = 'l',
        long = "logging",
        value_name = "LEVEL",
        default_value = "warn"
    )]
    log_level: LogLevel,
}

#[derive(ValueEnum, Clone, Debug, Copy)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }
    }
}

fn init_logging(log_level: &str) {
    // First try to use the environment variable if set, otherwise use the CLI argument
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level));

    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Starting Revenue Report application");

    // For now, always run terminal UI
    // In the future, this will check command line arguments to determine
    // whether to run terminal UI or web server
    let _repo = ui::terminal::run_terminal_ui()?;

    // Repository is loaded and ready for future use
    // (currently just exits, but this prepares for future functionality)

    tracing::info!("Application shutting down");

    Ok(())
}

fn main() {
    let args = Args::parse();

    init_logging(args.log_level.as_str());

    if let Err(e) = run() {
        tracing::error!(error = %e, "Application error");
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

mod db;
mod logging;
mod models;
mod services;
mod ui;
mod utils;

use clap::Parser;

use logging::LogLevel;

#[derive(Parser, Debug)]
#[command(name = "rev-report")]
#[command(about = "Revenue Report CSV Data Loader", long_about = None)]
struct Args {
    /// Set the logging level (trace, debug, info, warn, error)
    #[arg(
        short = 'l',
        long = "logging",
        value_name = "LEVEL",
        default_value = "error"
    )]
    log_level: LogLevel,
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

    logging::init_logging(args.log_level.as_str());

    if let Err(e) = run() {
        tracing::error!(error = %e, "Application error");
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

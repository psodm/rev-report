mod db;
mod models;
mod services;
mod ui;
mod utils;

fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
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
    init_logging();

    if let Err(e) = run() {
        tracing::error!(error = %e, "Application error");
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

mod db;
mod models;
mod services;
mod ui;
mod utils;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // For now, always run terminal UI
    // In the future, this will check command line arguments to determine
    // whether to run terminal UI or web server
    let _repo = ui::terminal::run_terminal_ui()?;

    // Repository is loaded and ready for future use
    // (currently just exits, but this prepares for future functionality)

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

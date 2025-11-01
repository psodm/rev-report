use dialoguer::Input;
use rev_report::db::csv_loader::load_data_from_csvs;
use rev_report::db::repositories::in_memory_repository::InMemoryRepository;
use std::path::Path;

fn validate_csv_file_path(file_path: &str) -> Result<(), String> {
    // Check if file exists
    if !Path::new(file_path).exists() {
        return Err(format!("File '{}' does not exist. Please provide a valid file path.", file_path));
    }

    // Check if file has .csv extension
    if !file_path.to_lowercase().ends_with(".csv") {
        return Err(format!("File '{}' is not a CSV file. Please provide a file with .csv extension.", file_path));
    }

    Ok(())
}

fn prompt_for_csv_file(prompt_message: &str) -> String {
    loop {
        let file_path: String = Input::new()
            .with_prompt(prompt_message)
            .interact_text()
            .unwrap_or_default();

        if file_path.trim().is_empty() {
            println!("❌ Error: Please provide a file path.");
            continue;
        }

        match validate_csv_file_path(&file_path) {
            Ok(()) => {
                println!("✅ File '{}' is valid.", file_path);
                return file_path;
            }
            Err(e) => {
                println!("❌ {}", e);
                continue;
            }
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║          Revenue Report CSV Data Loader                  ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    // Prompt for projects CSV file
    let projects_path = prompt_for_csv_file("Enter the path to the projects CSV file");
    println!();

    // Prompt for forecasts CSV file
    let forecasts_path = prompt_for_csv_file("Enter the path to the forecasts CSV file");
    println!();

    // Create repository
    println!("📦 Creating in-memory repository...");
    let repo = InMemoryRepository::new();
    println!("✅ Repository created successfully.");
    println!();

    // Load CSV files
    println!("📥 Loading CSV files into repository...");
    match load_data_from_csvs(&repo, &projects_path, &forecasts_path) {
        Ok((project_count, forecast_count)) => {
            println!("✅ Successfully loaded data:");
            println!("   • {} projects loaded from '{}'", project_count, projects_path);
            println!("   • {} forecasts loaded from '{}'", forecast_count, forecasts_path);
            println!();
            println!("🎉 Data loading completed successfully!");
        }
        Err(e) => {
            println!();
            println!("❌ Error loading CSV files:");
            println!("   {}", e);
            println!();
            println!("💡 This may indicate:");
            println!("   • Files are not properly formatted CSV files");
            println!("   • Files do not match the expected structure");
            println!("   • Data cannot be deserialized into Project/Forecast structs");
            println!();
            return Err(format!("Failed to load CSV files: {}", e).into());
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

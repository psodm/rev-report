use dialoguer::{Input, Select};
use rev_report::db::csv_loader::load_data_from_csvs;
use rev_report::db::repositories::in_memory_repository::InMemoryRepository;
use rev_report::db::repositories::project_repository::ProjectRepositoryTrait;
use rev_report::services::project_service::ProjectService;
use std::path::Path;

/// Safely truncates a string to a maximum number of characters (not bytes)
/// This handles multi-byte UTF-8 characters correctly
fn truncate_string(s: &str, max_chars: usize) -> &str {
    if s.chars().count() <= max_chars {
        s
    } else {
        s.char_indices()
            .nth(max_chars)
            .map(|(idx, _)| &s[..idx])
            .unwrap_or(s)
    }
}

fn validate_csv_file_path(file_path: &str) -> Result<(), String> {
    // Check if file exists
    if !Path::new(file_path).exists() {
        return Err(format!(
            "File '{}' does not exist. Please provide a valid file path.",
            file_path
        ));
    }

    // Check if file has .csv extension
    if !file_path.to_lowercase().ends_with(".csv") {
        return Err(format!(
            "File '{}' is not a CSV file. Please provide a file with .csv extension.",
            file_path
        ));
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

pub fn run_terminal_ui() -> Result<InMemoryRepository, Box<dyn std::error::Error>> {
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
            println!(
                "   • {} projects loaded from '{}'",
                project_count, projects_path
            );
            println!(
                "   • {} forecasts loaded from '{}'",
                forecast_count, forecasts_path
            );
            println!();
            println!("🎉 Data loading completed successfully!");
            println!();

            // Remove project S00000072027 if it exists (this project never has a forecast)
            if ProjectRepositoryTrait::delete_by_id(&repo, "S00000072027") {
                println!("ℹ️  Removed project S00000072027 (project never has a forecast)");
                println!();
            }

            // Create project service
            let project_service = ProjectService::new(&repo);

            // Show main menu
            show_main_menu(&project_service);

            Ok(repo)
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
            Err(format!("Failed to load CSV files: {}", e).into())
        }
    }
}

fn display_projects_without_forecasts(project_service: &ProjectService) {
    let projects = project_service.find_projects_without_forecasts();

    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║        Projects Without Forecasts                        ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    if projects.is_empty() {
        println!("✅ All projects have forecasts. No projects found without forecasts.");
    } else {
        println!("Found {} project(s) without forecasts:\n", projects.len());
        println!("{:-<193}", "");
        println!(
            "{:<15} {:<48} {:<98} {:<32}",
            "Project ID", "Customer", "Project Name", "Project Manager"
        );
        println!("{:-<193}", "");

        for project in projects {
            // Truncate customer name to max 44 characters
            let customer_name = truncate_string(&project.end_customer_name, 44);

            // Truncate project name to max 94 characters
            let project_name = truncate_string(&project.project_name, 94);

            // Extract just the name from project manager (remove account id part)
            let project_manager_name =
                ProjectService::extract_project_manager_name(&project.project_manager);

            println!(
                "{:<15} {:<48} {:<98} {:<32}",
                project.project_id, customer_name, project_name, project_manager_name
            );
        }

        println!("{:-<193}", "");
        println!();
    }
}

fn display_on_hold_projects(project_service: &ProjectService) {
    let projects = project_service.find_on_hold_projects();

    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║              On-Hold Projects                           ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    if projects.is_empty() {
        println!("✅ No projects are currently on hold.");
    } else {
        println!("Found {} on-hold project(s):\n", projects.len());
        println!("{:-<321}", "");
        println!(
            "{:<15} {:<48} {:<98} {:<32} {:<128}",
            "Project ID", "Customer", "Project Name", "Project Manager", "On Hold Comment"
        );
        println!("{:-<321}", "");

        for project in projects {
            // Truncate customer name to max 44 characters
            let customer_name = truncate_string(&project.end_customer_name, 44);

            // Truncate project name to max 94 characters
            let project_name = truncate_string(&project.project_name, 94);

            // Extract just the name from project manager (remove account id part)
            let project_manager_full_name =
                ProjectService::extract_project_manager_name(&project.project_manager);
            // Truncate project manager name to max 32 characters
            let project_manager_name = truncate_string(project_manager_full_name, 32);

            // Get on-hold comment or empty string, sanitize newlines, then truncate to max 128 characters
            let on_hold_comment = project.on_hold_comment.as_deref().unwrap_or("");
            let sanitized_comment = ProjectService::sanitize_on_hold_comment(on_hold_comment);
            let comment_display = truncate_string(&sanitized_comment, 128);

            println!(
                "{:<15} {:<48} {:<98} {:<32} {:<128}",
                project.project_id,
                customer_name,
                project_name,
                project_manager_name,
                comment_display
            );
        }

        println!("{:-<321}", "");
        println!();
    }
}

pub fn show_main_menu(project_service: &ProjectService) {
    loop {
        println!();
        println!("╔══════════════════════════════════════════════════════════╗");
        println!("║                   Main Menu                              ║");
        println!("╚══════════════════════════════════════════════════════════╝");
        println!();

        let options = vec![
            "Show projects with no forecast",
            "Show on-hold projects",
            "Exit",
        ];

        let selection = Select::new()
            .with_prompt("Select an option")
            .items(&options)
            .default(0)
            .interact()
            .unwrap_or(0);

        match selection {
            0 => {
                // Show projects with no forecast
                display_projects_without_forecasts(project_service);
            }
            1 => {
                // Show on-hold projects
                display_on_hold_projects(project_service);
            }
            2 => {
                // Exit
                println!();
                println!("👋 Goodbye!");
                break;
            }
            _ => {
                println!("❌ Invalid selection");
            }
        }
    }
}

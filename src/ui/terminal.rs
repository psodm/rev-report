use crate::db::csv_loader::load_data_from_csvs;
use crate::db::repositories::in_memory_repository::InMemoryRepository;
use crate::db::repositories::project_repository::ProjectRepositoryTrait;
use crate::models::project::Project;
use crate::ods::save_forecast_by_sales_org;
use crate::services::forecast_service::ForecastService;
use crate::services::project_service::ProjectService;
use crate::utils::utils::{format_currency, truncate_string};
use dialoguer::{Confirm, Input, Select};
use std::path::Path;
use tracing::{debug, error, info, trace, warn};

fn validate_csv_file_path(file_path: &str) -> Result<(), String> {
    trace!(file_path = %file_path, "Validating CSV file path");
    // Check if file exists
    if !Path::new(file_path).exists() {
        warn!(file_path = %file_path, "CSV file does not exist");
        return Err(format!(
            "File '{}' does not exist. Please provide a valid file path.",
            file_path
        ));
    }

    // Check if file has .csv extension
    if !file_path.to_lowercase().ends_with(".csv") {
        warn!(file_path = %file_path, "File does not have .csv extension");
        return Err(format!(
            "File '{}' is not a CSV file. Please provide a file with .csv extension.",
            file_path
        ));
    }

    debug!(file_path = %file_path, "CSV file path validated successfully");
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
    let span = tracing::info_span!("terminal_ui", operation = "run_terminal_ui");
    let _guard = span.enter();

    info!("Starting terminal UI");
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║          Revenue Report CSV Data Loader                  ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    // Prompt for projects CSV file
    let projects_path = prompt_for_csv_file("Enter the path to the projects CSV file");
    info!(projects_path = %projects_path, "Projects CSV file path obtained from user");
    println!();

    // Prompt for forecasts CSV file
    let forecasts_path = prompt_for_csv_file("Enter the path to the forecasts CSV file");
    info!(forecasts_path = %forecasts_path, "Forecasts CSV file path obtained from user");
    println!();

    // Create repository
    println!("📦 Creating in-memory repository...");
    let repo = InMemoryRepository::new();
    info!("Repository created successfully");
    println!("✅ Repository created successfully.");
    println!();

    // Load CSV files
    println!("📥 Loading CSV files into repository...");
    let load_span = tracing::info_span!("load_csv_data", projects_file = %projects_path, forecasts_file = %forecasts_path);
    let _load_guard = load_span.enter();
    match load_data_from_csvs(&repo, &projects_path, &forecasts_path) {
        Ok((project_count, forecast_count)) => {
            info!(
                project_count = project_count,
                forecast_count = forecast_count,
                "Successfully loaded CSV data"
            );
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
                info!(
                    project_id = "S00000072027",
                    "Removed project that never has a forecast"
                );
                println!("ℹ️  Removed project S00000072027 (project never has a forecast)");
                println!();
            }

            // Create services
            let project_service = ProjectService::new(&repo);
            let forecast_service = ForecastService::new(&repo);
            debug!("Services created successfully");

            // Show main menu
            show_main_menu(&project_service, &forecast_service);

            Ok(repo)
        }
        Err(e) => {
            error!(
                error = %e,
                projects_path = %projects_path,
                forecasts_path = %forecasts_path,
                "Failed to load CSV files"
            );
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
    let span = tracing::info_span!("display_projects_without_forecasts");
    let _guard = span.enter();
    let projects = project_service.find_projects_without_forecasts();

    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║        Projects Without Forecast                         ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    if projects.is_empty() {
        info!("No projects without forecast found");
        println!("✅ All projects have forecasts. No projects found without forecasts.");
    } else {
        info!(count = projects.len(), "Found projects without forecasts");
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
    let span = tracing::info_span!("display_on_hold_projects");
    let _guard = span.enter();
    use crate::db::repositories::forecast_repository::ForecastRepositoryTrait;

    let mut projects = project_service.find_on_hold_projects();

    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║              On-Hold Projects                           ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    debug!(count = projects.len(), "Displaying on-hold projects");

    if projects.is_empty() {
        info!("No on-hold projects found");
        println!("✅ No projects are currently on hold.");
    } else {
        info!(count = projects.len(), "Found on-hold projects");
        // Sort projects by class before displaying
        projects.sort_by(|a, b| {
            let class_a =
                ForecastRepositoryTrait::find_by_id(project_service.repository, &a.project_id)
                    .map(|f| f.class.clone())
                    .unwrap_or_else(|| String::new());

            let class_b =
                ForecastRepositoryTrait::find_by_id(project_service.repository, &b.project_id)
                    .map(|f| f.class.clone())
                    .unwrap_or_else(|| String::new());

            class_a.cmp(&class_b)
        });

        println!("Found {} on-hold project(s):\n", projects.len());
        println!("{:-<331}", "");
        println!(
            "{:<15} {:<48} {:<98} {:<32} {:<10} {:<128}",
            "Project ID", "Customer", "Project Name", "Project Manager", "Class", "On Hold Comment"
        );
        println!("{:-<331}", "");

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

            // Get class from forecast if available
            let class_display = ForecastRepositoryTrait::find_by_id(
                project_service.repository,
                &project.project_id,
            )
            .map(|f| f.class.clone())
            .unwrap_or_else(|| "N/A".to_string());

            // Get on-hold comment or empty string, sanitize newlines, then truncate to max 128 characters
            let on_hold_comment = project.on_hold_comment.as_deref().unwrap_or("");
            let sanitized_comment = ProjectService::sanitize_on_hold_comment(on_hold_comment);
            let comment_display = truncate_string(&sanitized_comment, 128);

            println!(
                "{:<15} {:<48} {:<98} {:<32} {:<10} {:<128}",
                project.project_id,
                customer_name,
                project_name,
                project_manager_name,
                class_display,
                comment_display
            );
        }

        println!("{:-<331}", "");
        println!();
    }
}

fn display_projects_without_account_executive(project_service: &ProjectService) {
    let span = tracing::info_span!("display_projects_without_account_executive");
    let _guard = span.enter();

    let mut projects = project_service.find_projects_without_account_executive();

    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║       Projects without Account Executive Listed          ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    debug!(
        count = projects.len(),
        "Displaying projects without Account Executive"
    );

    if projects.is_empty() {
        info!("No projects without Account Executive found");
        println!("✅ No projects without Account Executive found.");
        println!();
        return;
    }

    info!(
        count = projects.len(),
        "Found projects without Account Executive"
    );

    // Sort projects by Project Manager name
    projects.sort_by(|a, b| {
        let manager_a = ProjectService::extract_project_manager_name(&a.project_manager);
        let manager_b = ProjectService::extract_project_manager_name(&b.project_manager);
        manager_a.cmp(&manager_b)
    });

    println!(
        "Found {} project(s) without Account Executive:\n",
        projects.len()
    );

    // Single table header
    println!("{:-<193}", "");
    println!(
        "{:<15} {:<48} {:<98} {:<32}",
        "Project ID", "Customer", "Project", "Project Manager"
    );
    println!("{:-<193}", "");

    // Print all projects in a single table, ordered by Project Manager
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

        println!(
            "{:<15} {:<48} {:<98} {:<32}",
            project.project_id, customer_name, project_name, project_manager_name
        );
    }

    println!("{:-<193}", "");
    println!();
}

fn display_on_hold_projects_by_account_executive(project_service: &ProjectService) {
    let span = tracing::info_span!("display_on_hold_projects_by_account_executive");
    let _guard = span.enter();
    use crate::db::repositories::forecast_repository::ForecastRepositoryTrait;

    let grouped = project_service.get_on_hold_projects_by_account_executive();

    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║        On-Hold Projects by Account Executive             ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    if grouped.is_empty() {
        info!("No on-hold projects found");
        println!("✅ No on-hold projects found.");
        println!();
        return;
    }

    // Convert to vector and sort by Account Executive name
    let mut ae_groups: Vec<(String, Vec<Project>)> = grouped.into_iter().collect();
    ae_groups.sort_by(|a, b| a.0.cmp(&b.0));

    let total_count: usize = ae_groups.iter().map(|(_, projects)| projects.len()).sum();
    info!(
        total_count = total_count,
        ae_count = ae_groups.len(),
        "Found on-hold projects grouped by Account Executive"
    );
    println!(
        "Found {} on-hold project(s) across {} Account Executive(s):\n",
        total_count,
        ae_groups.len()
    );

    for (account_executive, mut projects) in ae_groups {
        println!("═══════════════════════════════════════════════════════════");
        println!("Account Executive: {}", account_executive);
        println!("═══════════════════════════════════════════════════════════");
        println!();

        // Sort projects by class before displaying
        projects.sort_by(|a, b| {
            let class_a =
                ForecastRepositoryTrait::find_by_id(project_service.repository, &a.project_id)
                    .map(|f| f.class.clone())
                    .unwrap_or_else(|| String::new());

            let class_b =
                ForecastRepositoryTrait::find_by_id(project_service.repository, &b.project_id)
                    .map(|f| f.class.clone())
                    .unwrap_or_else(|| String::new());

            class_a.cmp(&class_b)
        });

        println!("{:-<331}", "");
        println!(
            "{:<15} {:<48} {:<98} {:<32} {:<10} {:<128}",
            "Project ID", "Customer", "Project Name", "Project Manager", "Class", "On Hold Comment"
        );
        println!("{:-<331}", "");

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

            // Get class from forecast if available
            let class_display = ForecastRepositoryTrait::find_by_id(
                project_service.repository,
                &project.project_id,
            )
            .map(|f| f.class.clone())
            .unwrap_or_else(|| "N/A".to_string());

            // Get on-hold comment or empty string, sanitize newlines, then truncate to max 128 characters
            let on_hold_comment = project.on_hold_comment.as_deref().unwrap_or("");
            let sanitized_comment = ProjectService::sanitize_on_hold_comment(on_hold_comment);
            let comment_display = truncate_string(&sanitized_comment, 128);

            println!(
                "{:<15} {:<48} {:<98} {:<32} {:<10} {:<128}",
                project.project_id,
                customer_name,
                project_name,
                project_manager_name,
                class_display,
                comment_display
            );
        }

        println!("{:-<331}", "");
        println!();
    }
}

fn display_all_account_executives(project_service: &ProjectService) {
    let span = tracing::info_span!("display_all_account_executives");
    let _guard = span.enter();
    let account_executives = project_service.find_all_account_executives();

    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║              All Account Executives                      ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    debug!(
        count = account_executives.len(),
        "Displaying Account Executives"
    );

    if account_executives.is_empty() {
        info!("No Account Executives found");
        println!("✅ No Account Executives found.");
        println!();
        return;
    }

    info!(count = account_executives.len(), "Found Account Executives");
    println!("Found {} Account Executive(s):\n", account_executives.len());

    for (index, ae) in account_executives.iter().enumerate() {
        println!("{}. {}", index + 1, ae);
    }

    println!();
}

fn display_forecasts_by_project_manager(forecast_service: &ForecastService) {
    let span = tracing::info_span!("display_forecasts_by_project_manager");
    let _guard = span.enter();
    let forecasts_by_manager = forecast_service.get_forecasts_by_project_manager();

    debug!(
        manager_count = forecasts_by_manager.len(),
        "Displaying forecasts by Project Manager"
    );

    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║         Forecasts by Project Manager                     ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    if forecasts_by_manager.is_empty() {
        info!("No forecasts found");
        println!("✅ No forecasts found.");
        println!();
        return;
    }

    info!(
        manager_count = forecasts_by_manager.len(),
        "Found forecasts grouped by Project Manager"
    );

    for manager_group in forecasts_by_manager {
        println!("═══════════════════════════════════════════════════════════");
        println!("Project Manager: {}", manager_group.project_manager);
        println!("═══════════════════════════════════════════════════════════");
        println!();

        // Calculate totals for this project manager
        let mut month_totals = [0.0; 6];
        // Get currency code from first forecast (assuming all forecasts for a manager have same currency)
        let currency_code = manager_group
            .forecasts
            .first()
            .map(|(f, _)| f.currency.as_str())
            .unwrap_or("USD");

        // Table header
        println!("{:-<255}", "");
        println!(
            "{:<15} {:<48} {:<94} {:>15} {:>15} {:>15} {:>15} {:>15} {:>15}",
            "Project ID",
            "Customer",
            "Project Name",
            "Month 1",
            "Month 2",
            "Month 3",
            "Month 4",
            "Month 5",
            "Month 6"
        );
        println!("{:-<255}", "");

        for (forecast, customer_name) in &manager_group.forecasts {
            // Truncate customer name to max 44 characters
            let customer_display = customer_name
                .as_ref()
                .map(|c| truncate_string(c, 44))
                .unwrap_or("N/A");

            // Truncate project name to max 94 characters
            let project_name_display = truncate_string(&forecast.project_name, 94);

            // Format currency values with thousand separators and currency symbol
            let month1 = format_currency(forecast.month1_labor_revenue_commit, &forecast.currency);
            let month2 = format_currency(forecast.month2_labor_revenue_commit, &forecast.currency);
            let month3 = format_currency(forecast.month3_labor_revenue_commit, &forecast.currency);
            let month4 = format_currency(forecast.month4_labor_revenue_commit, &forecast.currency);
            let month5 = format_currency(forecast.month5_labor_revenue_commit, &forecast.currency);
            let month6 = format_currency(forecast.month6_labor_revenue_commit, &forecast.currency);

            // Add to totals
            month_totals[0] += forecast.month1_labor_revenue_commit;
            month_totals[1] += forecast.month2_labor_revenue_commit;
            month_totals[2] += forecast.month3_labor_revenue_commit;
            month_totals[3] += forecast.month4_labor_revenue_commit;
            month_totals[4] += forecast.month5_labor_revenue_commit;
            month_totals[5] += forecast.month6_labor_revenue_commit;

            println!(
                "{:<15} {:<48} {:<94} {:>15} {:>15} {:>15} {:>15} {:>15} {:>15}",
                forecast.project_id,
                customer_display,
                project_name_display,
                month1,
                month2,
                month3,
                month4,
                month5,
                month6
            );
        }

        // Print totals row
        println!("{:-<255}", "");
        println!(
            "{:<15} {:<48} {:<94} {:>15} {:>15} {:>15} {:>15} {:>15} {:>15}",
            "",
            "TOTAL",
            "",
            format_currency(month_totals[0], currency_code),
            format_currency(month_totals[1], currency_code),
            format_currency(month_totals[2], currency_code),
            format_currency(month_totals[3], currency_code),
            format_currency(month_totals[4], currency_code),
            format_currency(month_totals[5], currency_code)
        );
        println!("{:-<255}", "");
        println!();
    }
}

fn display_projects_by_project_manager(project_service: &ProjectService) {
    let span = tracing::info_span!("display_projects_by_project_manager");
    let _guard = span.enter();
    let projects_by_manager = project_service.get_projects_by_project_manager();

    debug!(
        manager_count = projects_by_manager.len(),
        "Displaying projects by Project Manager"
    );

    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║         All Projects by Project Manager                  ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    if projects_by_manager.is_empty() {
        info!("No projects found");
        println!("✅ No projects found.");
        println!();
        return;
    }

    info!(
        manager_count = projects_by_manager.len(),
        "Found projects grouped by Project Manager"
    );

    for manager_group in projects_by_manager {
        println!("═══════════════════════════════════════════════════════════");
        println!("Project Manager: {}", manager_group.project_manager);
        println!("═══════════════════════════════════════════════════════════");
        println!();

        // Table header
        println!("{:-<238}", "");
        println!(
            "{:<15} {:<48} {:<94} {:<12} {:<12} {:<10} {:>20} {:>20}",
            "Project ID",
            "Customer",
            "Project Name",
            "Start Date",
            "End Date",
            "Class",
            "Total Contract",
            "Remaining Contract"
        );
        println!("{:-<238}", "");

        for (project, contract_values) in &manager_group.projects {
            // Truncate customer name to max 44 characters
            let customer_display = truncate_string(&project.end_customer_name, 44);

            // Truncate project name to max 94 characters
            let project_name_display = truncate_string(&project.project_name, 94);

            // Format dates
            let start_date_display = &project.start_date;
            let end_date_display = &project.end_date;

            // Format contract values with currency formatting and get class
            let (total_contract, remaining_contract, class_display) = match contract_values {
                Some((total, remaining, currency, class)) => (
                    format_currency(*total, currency),
                    format_currency(*remaining, currency),
                    class.clone(),
                ),
                None => ("N/A".to_string(), "N/A".to_string(), "N/A".to_string()),
            };

            println!(
                "{:<15} {:<48} {:<94} {:<12} {:<12} {:<10} {:>20} {:>20}",
                project.project_id,
                customer_display,
                project_name_display,
                start_date_display,
                end_date_display,
                class_display,
                total_contract,
                remaining_contract
            );
        }

        println!("{:-<238}", "");
        println!();
    }
}

fn display_revenue_summary_by_account_executive(forecast_service: &ForecastService) {
    let span = tracing::info_span!("display_revenue_summary_by_account_executive");
    let _guard = span.enter();

    let summaries = forecast_service.get_revenue_summary_by_account_executive();

    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║        Revenue Summary by Account Executive              ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    if summaries.is_empty() {
        info!("No revenue summaries found");
        println!("✅ No revenue summaries found.");
        println!();
        return;
    }

    let total_count = summaries.len();
    info!(
        ae_count = total_count,
        "Displaying revenue summaries by Account Executive"
    );
    println!("Found {} Account Executive(s):\n", total_count);

    // Get a representative currency for formatting (use USD as default)
    // We'll use the currency from the first forecast we find, or default to USD
    let default_currency = "USD";

    println!("{:-<154}", "");
    println!(
        "{:<40} {:>18} {:>18} {:>18} {:>18} {:>18} {:>18}",
        "Account Executive", "Month 1", "Month 2", "Month 3", "Month 4", "Month 5", "Month 6"
    );
    println!("{:-<154}", "");

    for summary in summaries {
        println!(
            "{:<40} {:>18} {:>18} {:>18} {:>18} {:>18} {:>18}",
            truncate_string(&summary.account_executive, 38),
            format_currency(summary.month1_total, default_currency),
            format_currency(summary.month2_total, default_currency),
            format_currency(summary.month3_total, default_currency),
            format_currency(summary.month4_total, default_currency),
            format_currency(summary.month5_total, default_currency),
            format_currency(summary.month6_total, default_currency),
        );
    }

    println!("{:-<154}", "");
    println!();

    debug!(
        ae_count = total_count,
        "Displayed revenue summaries by Account Executive"
    );
}

fn display_total_revenue_forecast(forecast_service: &ForecastService) {
    let span = tracing::info_span!("display_total_revenue_forecast");
    let _guard = span.enter();

    let (month1_total, month2_total, month3_total, month4_total, month5_total, month6_total) =
        forecast_service.get_total_revenue_forecast();

    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║           Total Revenue Forecast (All Projects)          ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    // Use USD as default currency for formatting
    let default_currency = "USD";

    println!("{:-<150}", "");
    println!(
        "{:<20} {:>18} {:>18} {:>18} {:>18} {:>18} {:>18}",
        "Total Revenue", "Month 1", "Month 2", "Month 3", "Month 4", "Month 5", "Month 6"
    );
    println!("{:-<150}", "");

    println!(
        "{:<20} {:>18} {:>18} {:>18} {:>18} {:>18} {:>18}",
        "",
        format_currency(month1_total, default_currency),
        format_currency(month2_total, default_currency),
        format_currency(month3_total, default_currency),
        format_currency(month4_total, default_currency),
        format_currency(month5_total, default_currency),
        format_currency(month6_total, default_currency),
    );

    println!("{:-<150}", "");
    println!();

    info!(
        month1 = month1_total,
        month2 = month2_total,
        month3 = month3_total,
        month4 = month4_total,
        month5 = month5_total,
        month6 = month6_total,
        "Displayed total revenue forecast"
    );
}

fn display_forecast_by_sales_org(forecast_service: &ForecastService) {
    let span = tracing::info_span!("display_forecast_by_sales_org");
    let _guard = span.enter();

    let summaries = forecast_service.get_revenue_summary_by_sales_org();

    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║              Forecast for Sales Orgs                      ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    if summaries.is_empty() {
        info!("No revenue summaries found");
        println!("✅ No revenue summaries found.");
        println!();
        return;
    }

    let total_count = summaries.len();
    info!(
        sales_org_count = total_count,
        "Displaying revenue summaries by Sales Org"
    );
    println!("Found {} Sales Org(s):\n", total_count);

    // Use USD as default currency for formatting
    let default_currency = "USD";

    println!("{:-<150}", "");
    println!(
        "{:<30} {:>18} {:>18} {:>18} {:>18} {:>18} {:>18}",
        "Sales Org", "Month 1", "Month 2", "Month 3", "Month 4", "Month 5", "Month 6"
    );
    println!("{:-<150}", "");

    for summary in &summaries {
        println!(
            "{:<30} {:>18} {:>18} {:>18} {:>18} {:>18} {:>18}",
            truncate_string(&summary.sales_org, 28),
            format_currency(summary.month1_total, default_currency),
            format_currency(summary.month2_total, default_currency),
            format_currency(summary.month3_total, default_currency),
            format_currency(summary.month4_total, default_currency),
            format_currency(summary.month5_total, default_currency),
            format_currency(summary.month6_total, default_currency),
        );
    }

    println!("{:-<150}", "");
    println!();

    match Confirm::new()
        .with_prompt("Save results to forecast_by_sales_org.ods?")
        .default(false)
        .interact()
    {
        Ok(true) => match save_forecast_by_sales_org(&summaries) {
            Ok(path) => {
                println!("✅ Saved results to {}", path.display());
                println!();
            }
            Err(err) => {
                error!(error = %err, "Failed to save Sales Org forecast ODS report");
                println!("❌ Failed to save ODS file: {}", err);
                println!();
            }
        },
        Ok(false) => {
            info!("User chose not to save Sales Org forecast to ODS");
        }
        Err(err) => {
            error!(error = %err, "Failed to read confirmation input for ODS export");
            println!("❌ Could not read confirmation input: {}", err);
            println!();
        }
    }

    debug!(
        sales_org_count = total_count,
        "Displayed revenue summaries by Sales Org"
    );
}

pub fn show_main_menu(project_service: &ProjectService, forecast_service: &ForecastService) {
    let span = tracing::info_span!("main_menu");
    let _guard = span.enter();
    info!("Main menu loop started");

    loop {
        println!();
        println!("╔══════════════════════════════════════════════════════════╗");
        println!("║                   Main Menu                              ║");
        println!("╚══════════════════════════════════════════════════════════╝");
        println!();

        let options = vec![
            "Projects with no forecast",
            "On-hold projects",
            "On-hold projects by Account Executive",
            "Projects without Account Executive",
            "All Account Executives",
            "Forecasts by Project Manager",
            "All projects by Project Manager",
            "Revenue summary for Account Executive",
            "Total revenue forecast (all projects)",
            "Forecast for Sales Orgs",
            "Exit",
        ];

        let selection = Select::new()
            .with_prompt("Select an option")
            .items(&options)
            .default(0)
            .interact();

        match selection {
            Ok(0) => {
                // Show projects with no forecast
                info!("User selected: Show projects with no forecast");
                display_projects_without_forecasts(project_service);
            }
            Ok(1) => {
                // Show on-hold projects
                info!("User selected: Show on-hold projects");
                display_on_hold_projects(project_service);
            }
            Ok(2) => {
                // Show on-hold projects by Account Executive
                info!("User selected: Show on-hold projects by Account Executive");
                display_on_hold_projects_by_account_executive(project_service);
            }
            Ok(3) => {
                // Show projects without Account Executive
                info!("User selected: Show projects without Account Executive");
                display_projects_without_account_executive(project_service);
            }
            Ok(4) => {
                // Show all Account Executives
                info!("User selected: Show all Account Executives");
                display_all_account_executives(project_service);
            }
            Ok(5) => {
                // Show forecasts by Project Manager
                info!("User selected: Show forecasts by Project Manager");
                display_forecasts_by_project_manager(forecast_service);
            }
            Ok(6) => {
                // Show all projects by Project Manager
                info!("User selected: Show all projects by Project Manager");
                display_projects_by_project_manager(project_service);
            }
            Ok(7) => {
                // Show revenue summary for Account Executive
                info!("User selected: Show revenue summary for Account Executive");
                display_revenue_summary_by_account_executive(forecast_service);
            }
            Ok(8) => {
                // Show total revenue forecast (all projects)
                info!("User selected: Show total revenue forecast (all projects)");
                display_total_revenue_forecast(forecast_service);
            }
            Ok(9) => {
                // Show forecast for Sales Orgs
                info!("User selected: Show forecast for Sales Orgs");
                display_forecast_by_sales_org(forecast_service);
            }
            Ok(10) => {
                // Exit
                info!("User selected: Exit - terminating application");
                println!();
                println!("👋 Goodbye!");
                break;
            }
            Err(e) => {
                warn!(error = %e, "Invalid menu selection");
                println!("❌ Invalid selection");
            }
            _ => {
                warn!("Unexpected menu selection");
                println!("❌ Invalid selection");
            }
        }
    }
}

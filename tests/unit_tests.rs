use rev_report::db::csv_loader::{load_data_from_csvs, load_forecasts_from_csv, load_projects_from_csv};
use rev_report::db::repositories::forecast_repository::{ForecastRepository, ForecastRepositoryTrait};
use rev_report::db::repositories::in_memory_repository::InMemoryRepository;
use rev_report::db::repositories::project_repository::{ProjectRepository, ProjectRepositoryTrait};
use rev_report::models::forecast::Forecast;
use rev_report::models::project::Project;
use rev_report::services::project_service::ProjectService;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

// Helper function to create a test Project
fn create_test_project(project_id: &str, project_name: &str) -> Project {
    Project {
        project_id: project_id.to_string(),
        end_customer_name: "Test Customer".to_string(),
        project_name: project_name.to_string(),
        project_manager: "Test Manager".to_string(),
        account_executive: Some("Test AE".to_string()),
        start_date: "2024-01-01".to_string(),
        end_date: "2024-12-31".to_string(),
        on_hold: false,
        on_hold_comment: None,
    }
}

// Helper function to create a test Forecast
fn create_test_forecast(project_id: &str, project_name: &str) -> Forecast {
    Forecast {
        sold_to_company: "Test Company".to_string(),
        project_manager: "Test Manager".to_string(),
        project_name: project_name.to_string(),
        project_id: project_id.to_string(),
        class: "Test Class".to_string(),
        start_date: "2024-01-01".to_string(),
        finish_date: "2024-12-31".to_string(),
        contract_total_value: 100000.0,
        contract_remaining_value: 75000.0,
        currency: "USD".to_string(),
        month1_labor_revenue_commit: 10000.0,
        month2_labor_revenue_commit: 15000.0,
        month3_labor_revenue_commit: 20000.0,
        month4_labor_revenue_commit: 15000.0,
        month5_labor_revenue_commit: 10000.0,
        month6_labor_revenue_commit: 5000.0,
    }
}

// ========== ProjectRepository Tests ==========

#[test]
fn test_project_repository_new() {
    let repo = ProjectRepository::new();
    let projects = repo.find_all();
    assert_eq!(projects.len(), 0);
}

#[test]
fn test_project_repository_insert() {
    let repo = ProjectRepository::new();
    let project = create_test_project("PROJ-001", "Test Project 1");
    repo.insert(project);
    
    let projects = repo.find_all();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].project_id, "PROJ-001");
}

#[test]
fn test_project_repository_insert_multiple() {
    let repo = ProjectRepository::new();
    let project1 = create_test_project("PROJ-001", "Test Project 1");
    let project2 = create_test_project("PROJ-002", "Test Project 2");
    let project3 = create_test_project("PROJ-003", "Test Project 3");
    
    repo.insert(project1);
    repo.insert(project2);
    repo.insert(project3);
    
    let projects = repo.find_all();
    assert_eq!(projects.len(), 3);
}

#[test]
fn test_project_repository_find_by_id_exists() {
    let repo = ProjectRepository::new();
    let project = create_test_project("PROJ-001", "Test Project 1");
    repo.insert(project);
    
    let found = repo.find_by_id("PROJ-001");
    assert!(found.is_some());
    let found_project = found.unwrap();
    assert_eq!(found_project.project_id, "PROJ-001");
    assert_eq!(found_project.project_name, "Test Project 1");
}

#[test]
fn test_project_repository_find_by_id_not_exists() {
    let repo = ProjectRepository::new();
    let project = create_test_project("PROJ-001", "Test Project 1");
    repo.insert(project);
    
    let found = repo.find_by_id("PROJ-999");
    assert!(found.is_none());
}

#[test]
fn test_project_repository_insert_updates_existing() {
    let repo = ProjectRepository::new();
    let project1 = create_test_project("PROJ-001", "Test Project 1");
    repo.insert(project1);
    
    // Insert a new project with the same ID but different name
    let mut project2 = create_test_project("PROJ-001", "Updated Project Name");
    project2.end_customer_name = "Updated Customer".to_string();
    repo.insert(project2);
    
    // Should only have one project, and it should be the updated one
    let projects = repo.find_all();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].project_name, "Updated Project Name");
    assert_eq!(projects[0].end_customer_name, "Updated Customer");
}

#[test]
fn test_project_repository_find_all_empty() {
    let repo = ProjectRepository::new();
    let projects = repo.find_all();
    assert_eq!(projects.len(), 0);
}

#[test]
fn test_project_repository_delete_by_id_exists() {
    let repo = ProjectRepository::new();
    let project1 = create_test_project("PROJ-001", "Test Project 1");
    let project2 = create_test_project("PROJ-002", "Test Project 2");
    repo.insert(project1);
    repo.insert(project2);
    
    // Verify both projects exist
    let projects = repo.find_all();
    assert_eq!(projects.len(), 2);
    
    // Delete one project
    let deleted = repo.delete_by_id("PROJ-001");
    assert!(deleted);
    
    // Verify it was deleted
    let projects_after = repo.find_all();
    assert_eq!(projects_after.len(), 1);
    assert_eq!(projects_after[0].project_id, "PROJ-002");
    
    // Verify it can't be found by ID
    let found = repo.find_by_id("PROJ-001");
    assert!(found.is_none());
}

#[test]
fn test_project_repository_delete_by_id_not_exists() {
    let repo = ProjectRepository::new();
    let project = create_test_project("PROJ-001", "Test Project 1");
    repo.insert(project);
    
    // Try to delete a project that doesn't exist
    let deleted = repo.delete_by_id("PROJ-999");
    assert!(!deleted);
    
    // Verify original project still exists
    let projects = repo.find_all();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].project_id, "PROJ-001");
}

#[test]
fn test_project_repository_delete_by_id_empty_repository() {
    let repo = ProjectRepository::new();
    
    // Try to delete from empty repository
    let deleted = repo.delete_by_id("PROJ-001");
    assert!(!deleted);
    
    // Verify repository is still empty
    let projects = repo.find_all();
    assert_eq!(projects.len(), 0);
}

#[test]
fn test_project_repository_delete_all_projects() {
    let repo = ProjectRepository::new();
    let project1 = create_test_project("PROJ-001", "Test Project 1");
    let project2 = create_test_project("PROJ-002", "Test Project 2");
    let project3 = create_test_project("PROJ-003", "Test Project 3");
    
    repo.insert(project1);
    repo.insert(project2);
    repo.insert(project3);
    
    // Delete all projects
    assert!(repo.delete_by_id("PROJ-001"));
    assert!(repo.delete_by_id("PROJ-002"));
    assert!(repo.delete_by_id("PROJ-003"));
    
    // Verify repository is empty
    let projects = repo.find_all();
    assert_eq!(projects.len(), 0);
}

// ========== ForecastRepository Tests ==========

#[test]
fn test_forecast_repository_new() {
    let repo = ForecastRepository::new();
    let forecasts = repo.find_all();
    assert_eq!(forecasts.len(), 0);
}

#[test]
fn test_forecast_repository_insert() {
    let repo = ForecastRepository::new();
    let forecast = create_test_forecast("PROJ-001", "Test Project 1");
    repo.insert(forecast);
    
    let forecasts = repo.find_all();
    assert_eq!(forecasts.len(), 1);
    assert_eq!(forecasts[0].project_id, "PROJ-001");
}

#[test]
fn test_forecast_repository_insert_multiple() {
    let repo = ForecastRepository::new();
    let forecast1 = create_test_forecast("PROJ-001", "Test Project 1");
    let forecast2 = create_test_forecast("PROJ-002", "Test Project 2");
    let forecast3 = create_test_forecast("PROJ-003", "Test Project 3");
    
    repo.insert(forecast1);
    repo.insert(forecast2);
    repo.insert(forecast3);
    
    let forecasts = repo.find_all();
    assert_eq!(forecasts.len(), 3);
}

#[test]
fn test_forecast_repository_find_by_id_exists() {
    let repo = ForecastRepository::new();
    let forecast = create_test_forecast("PROJ-001", "Test Project 1");
    repo.insert(forecast);
    
    let found = repo.find_by_id("PROJ-001");
    assert!(found.is_some());
    let found_forecast = found.unwrap();
    assert_eq!(found_forecast.project_id, "PROJ-001");
    assert_eq!(found_forecast.project_name, "Test Project 1");
    assert_eq!(found_forecast.contract_total_value, 100000.0);
}

#[test]
fn test_forecast_repository_find_by_id_not_exists() {
    let repo = ForecastRepository::new();
    let forecast = create_test_forecast("PROJ-001", "Test Project 1");
    repo.insert(forecast);
    
    let found = repo.find_by_id("PROJ-999");
    assert!(found.is_none());
}

#[test]
fn test_forecast_repository_insert_updates_existing() {
    let repo = ForecastRepository::new();
    let forecast1 = create_test_forecast("PROJ-001", "Test Project 1");
    repo.insert(forecast1);
    
    // Insert a new forecast with the same project_id but different values
    let mut forecast2 = create_test_forecast("PROJ-001", "Updated Project Name");
    forecast2.contract_total_value = 200000.0;
    forecast2.currency = "EUR".to_string();
    repo.insert(forecast2);
    
    // Should only have one forecast, and it should be the updated one
    let forecasts = repo.find_all();
    assert_eq!(forecasts.len(), 1);
    assert_eq!(forecasts[0].project_name, "Updated Project Name");
    assert_eq!(forecasts[0].contract_total_value, 200000.0);
    assert_eq!(forecasts[0].currency, "EUR");
}

#[test]
fn test_forecast_repository_find_all_empty() {
    let repo = ForecastRepository::new();
    let forecasts = repo.find_all();
    assert_eq!(forecasts.len(), 0);
}

// ========== InMemoryRepository Tests ==========

#[test]
fn test_in_memory_repository_new() {
    let repo = InMemoryRepository::new();
    let projects = ProjectRepositoryTrait::find_all(&repo);
    let forecasts = ForecastRepositoryTrait::find_all(&repo);
    assert_eq!(projects.len(), 0);
    assert_eq!(forecasts.len(), 0);
}

#[test]
fn test_in_memory_repository_project_insert() {
    let repo = InMemoryRepository::new();
    let project = create_test_project("PROJ-001", "Test Project 1");
    ProjectRepositoryTrait::insert(&repo, project);
    
    let projects = ProjectRepositoryTrait::find_all(&repo);
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].project_id, "PROJ-001");
}

#[test]
fn test_in_memory_repository_forecast_insert() {
    let repo = InMemoryRepository::new();
    let forecast = create_test_forecast("PROJ-001", "Test Project 1");
    ForecastRepositoryTrait::insert(&repo, forecast);
    
    let forecasts = ForecastRepositoryTrait::find_all(&repo);
    assert_eq!(forecasts.len(), 1);
    assert_eq!(forecasts[0].project_id, "PROJ-001");
}

#[test]
fn test_in_memory_repository_project_find_by_id() {
    let repo = InMemoryRepository::new();
    let project = create_test_project("PROJ-001", "Test Project 1");
    ProjectRepositoryTrait::insert(&repo, project);
    
    let found = ProjectRepositoryTrait::find_by_id(&repo, "PROJ-001");
    assert!(found.is_some());
    assert_eq!(found.unwrap().project_id, "PROJ-001");
    
    let not_found = ProjectRepositoryTrait::find_by_id(&repo, "PROJ-999");
    assert!(not_found.is_none());
}

#[test]
fn test_in_memory_repository_forecast_find_by_id() {
    let repo = InMemoryRepository::new();
    let forecast = create_test_forecast("PROJ-001", "Test Project 1");
    ForecastRepositoryTrait::insert(&repo, forecast);
    
    let found = ForecastRepositoryTrait::find_by_id(&repo, "PROJ-001");
    assert!(found.is_some());
    assert_eq!(found.unwrap().project_id, "PROJ-001");
    
    let not_found = ForecastRepositoryTrait::find_by_id(&repo, "PROJ-999");
    assert!(not_found.is_none());
}

#[test]
fn test_in_memory_repository_mixed_operations() {
    let repo = InMemoryRepository::new();
    
    // Insert projects and forecasts
    let project1 = create_test_project("PROJ-001", "Project 1");
    let project2 = create_test_project("PROJ-002", "Project 2");
    let forecast1 = create_test_forecast("PROJ-001", "Project 1");
    let forecast2 = create_test_forecast("PROJ-002", "Project 2");
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    ForecastRepositoryTrait::insert(&repo, forecast2);
    
    // Verify both repositories work independently
    let projects = ProjectRepositoryTrait::find_all(&repo);
    let forecasts = ForecastRepositoryTrait::find_all(&repo);
    assert_eq!(projects.len(), 2);
    assert_eq!(forecasts.len(), 2);
    
    // Verify finding by ID works for both
    let found_project = ProjectRepositoryTrait::find_by_id(&repo, "PROJ-001");
    let found_forecast = ForecastRepositoryTrait::find_by_id(&repo, "PROJ-001");
    assert!(found_project.is_some());
    assert!(found_forecast.is_some());
    assert_eq!(found_project.unwrap().project_id, "PROJ-001");
    assert_eq!(found_forecast.unwrap().project_id, "PROJ-001");
}

#[test]
fn test_in_memory_repository_project_update() {
    let repo = InMemoryRepository::new();
    let project1 = create_test_project("PROJ-001", "Original Name");
    ProjectRepositoryTrait::insert(&repo, project1);
    
    let mut project2 = create_test_project("PROJ-001", "Updated Name");
    project2.end_customer_name = "Updated Customer".to_string();
    ProjectRepositoryTrait::insert(&repo, project2);
    
    let found = ProjectRepositoryTrait::find_by_id(&repo, "PROJ-001");
    assert!(found.is_some());
    let found_project = found.unwrap();
    assert_eq!(found_project.project_name, "Updated Name");
    assert_eq!(found_project.end_customer_name, "Updated Customer");
}

#[test]
fn test_in_memory_repository_forecast_update() {
    let repo = InMemoryRepository::new();
    let forecast1 = create_test_forecast("PROJ-001", "Original Name");
    ForecastRepositoryTrait::insert(&repo, forecast1);
    
    let mut forecast2 = create_test_forecast("PROJ-001", "Updated Name");
    forecast2.contract_total_value = 500000.0;
    ForecastRepositoryTrait::insert(&repo, forecast2);
    
    let found = ForecastRepositoryTrait::find_by_id(&repo, "PROJ-001");
    assert!(found.is_some());
    let found_forecast = found.unwrap();
    assert_eq!(found_forecast.project_name, "Updated Name");
    assert_eq!(found_forecast.contract_total_value, 500000.0);
}

#[test]
fn test_in_memory_repository_project_delete_by_id_exists() {
    let repo = InMemoryRepository::new();
    let project1 = create_test_project("PROJ-001", "Test Project 1");
    let project2 = create_test_project("PROJ-002", "Test Project 2");
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    
    // Verify both projects exist
    let projects = ProjectRepositoryTrait::find_all(&repo);
    assert_eq!(projects.len(), 2);
    
    // Delete one project
    let deleted = ProjectRepositoryTrait::delete_by_id(&repo, "PROJ-001");
    assert!(deleted);
    
    // Verify it was deleted
    let projects_after = ProjectRepositoryTrait::find_all(&repo);
    assert_eq!(projects_after.len(), 1);
    assert_eq!(projects_after[0].project_id, "PROJ-002");
    
    // Verify it can't be found by ID
    let found = ProjectRepositoryTrait::find_by_id(&repo, "PROJ-001");
    assert!(found.is_none());
}

#[test]
fn test_in_memory_repository_project_delete_by_id_not_exists() {
    let repo = InMemoryRepository::new();
    let project = create_test_project("PROJ-001", "Test Project 1");
    ProjectRepositoryTrait::insert(&repo, project);
    
    // Try to delete a project that doesn't exist
    let deleted = ProjectRepositoryTrait::delete_by_id(&repo, "PROJ-999");
    assert!(!deleted);
    
    // Verify original project still exists
    let projects = ProjectRepositoryTrait::find_all(&repo);
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].project_id, "PROJ-001");
}

#[test]
fn test_in_memory_repository_project_delete_by_id_empty() {
    let repo = InMemoryRepository::new();
    
    // Try to delete from empty repository
    let deleted = ProjectRepositoryTrait::delete_by_id(&repo, "PROJ-001");
    assert!(!deleted);
    
    // Verify repository is still empty
    let projects = ProjectRepositoryTrait::find_all(&repo);
    assert_eq!(projects.len(), 0);
}

#[test]
fn test_in_memory_repository_project_delete_specific_id() {
    let repo = InMemoryRepository::new();
    // Insert the specific project that should be deleted
    let special_project = create_test_project("S00000072027", "Special Project");
    let project1 = create_test_project("PROJ-001", "Test Project 1");
    let project2 = create_test_project("PROJ-002", "Test Project 2");
    
    ProjectRepositoryTrait::insert(&repo, special_project);
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    
    // Verify all projects exist
    let projects = ProjectRepositoryTrait::find_all(&repo);
    assert_eq!(projects.len(), 3);
    
    // Delete the specific project ID
    let deleted = ProjectRepositoryTrait::delete_by_id(&repo, "S00000072027");
    assert!(deleted);
    
    // Verify it was deleted and others remain
    let projects_after = ProjectRepositoryTrait::find_all(&repo);
    assert_eq!(projects_after.len(), 2);
    
    // Verify the specific project can't be found
    let found = ProjectRepositoryTrait::find_by_id(&repo, "S00000072027");
    assert!(found.is_none());
    
    // Verify other projects still exist
    assert!(ProjectRepositoryTrait::find_by_id(&repo, "PROJ-001").is_some());
    assert!(ProjectRepositoryTrait::find_by_id(&repo, "PROJ-002").is_some());
}

// ========== ProjectService Tests ==========

#[test]
fn test_project_service_extract_project_manager_name_with_account_id() {
    let input = "Putra, Kim{kp055389@broadcom.net}";
    let result = ProjectService::extract_project_manager_name(input);
    assert_eq!(result, "Putra, Kim");
}

#[test]
fn test_project_service_extract_project_manager_name_without_account_id() {
    let input = "Smith, John";
    let result = ProjectService::extract_project_manager_name(input);
    assert_eq!(result, "Smith, John");
}

#[test]
fn test_project_service_extract_project_manager_name_empty_string() {
    let input = "";
    let result = ProjectService::extract_project_manager_name(input);
    assert_eq!(result, "");
}

#[test]
fn test_project_service_extract_project_manager_name_only_brace() {
    let input = "{kp055389@broadcom.net}";
    let result = ProjectService::extract_project_manager_name(input);
    assert_eq!(result, "");
}

#[test]
fn test_project_service_extract_project_manager_name_multiple_braces() {
    let input = "Last, First{account1}{account2}";
    let result = ProjectService::extract_project_manager_name(input);
    assert_eq!(result, "Last, First");
}

#[test]
fn test_project_service_extract_project_manager_name_complex_format() {
    let input = "Doe, Jane {jd123@company.com}";
    let result = ProjectService::extract_project_manager_name(input);
    assert_eq!(result, "Doe, Jane ");
}

#[test]
fn test_project_service_sanitize_on_hold_comment_with_newlines() {
    let input = "On hold - New Resources are in progress for onboarding";
    let result = ProjectService::sanitize_on_hold_comment(input);
    assert_eq!(result, "On hold - New Resources are in progress for onboarding");
}

#[test]
fn test_project_service_sanitize_on_hold_comment_with_single_newline() {
    let input = "On hold\nNew Resources are in progress";
    let result = ProjectService::sanitize_on_hold_comment(input);
    assert_eq!(result, "On hold New Resources are in progress");
}

#[test]
fn test_project_service_sanitize_on_hold_comment_with_multiple_newlines() {
    let input = "On hold\n\nNew Resources\nare in progress";
    let result = ProjectService::sanitize_on_hold_comment(input);
    assert_eq!(result, "On hold New Resources are in progress");
}

#[test]
fn test_project_service_sanitize_on_hold_comment_with_carriage_return() {
    let input = "On hold\rNew Resources are in progress";
    let result = ProjectService::sanitize_on_hold_comment(input);
    assert_eq!(result, "On hold New Resources are in progress");
}

#[test]
fn test_project_service_sanitize_on_hold_comment_with_newline_and_carriage_return() {
    let input = "On hold\r\nNew Resources are in progress";
    let result = ProjectService::sanitize_on_hold_comment(input);
    assert_eq!(result, "On hold New Resources are in progress");
}

#[test]
fn test_project_service_sanitize_on_hold_comment_empty_string() {
    let input = "";
    let result = ProjectService::sanitize_on_hold_comment(input);
    assert_eq!(result, "");
}

#[test]
fn test_project_service_sanitize_on_hold_comment_with_leading_trailing_whitespace() {
    let input = "  On hold - New Resources are in progress  ";
    let result = ProjectService::sanitize_on_hold_comment(input);
    assert_eq!(result, "On hold - New Resources are in progress");
}

#[test]
fn test_project_service_sanitize_on_hold_comment_with_newlines_and_whitespace() {
    let input = "  On hold\n\n  New Resources\n  are in progress  ";
    let result = ProjectService::sanitize_on_hold_comment(input);
    assert_eq!(result, "On hold New Resources are in progress");
}

#[test]
fn test_project_service_sanitize_on_hold_comment_no_newlines() {
    let input = "On hold - New Resources are in progress for onboarding";
    let result = ProjectService::sanitize_on_hold_comment(input);
    assert_eq!(result, "On hold - New Resources are in progress for onboarding");
}

#[test]
fn test_project_service_find_projects_without_forecasts() {
    let repo = InMemoryRepository::new();
    let project1 = create_test_project("PROJ-001", "Project 1");
    let project2 = create_test_project("PROJ-002", "Project 2");
    let project3 = create_test_project("PROJ-003", "Project 3");
    let forecast1 = create_test_forecast("PROJ-001", "Project 1");
    let forecast2 = create_test_forecast("PROJ-002", "Project 2");
    // PROJ-003 has no forecast
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ProjectRepositoryTrait::insert(&repo, project3);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    ForecastRepositoryTrait::insert(&repo, forecast2);
    
    let project_service = ProjectService::new(&repo);
    let projects_without_forecasts = project_service.find_projects_without_forecasts();
    
    assert_eq!(projects_without_forecasts.len(), 1);
    assert_eq!(projects_without_forecasts[0].project_id, "PROJ-003");
}

#[test]
fn test_project_service_find_projects_without_forecasts_all_have_forecasts() {
    let repo = InMemoryRepository::new();
    let project1 = create_test_project("PROJ-001", "Project 1");
    let project2 = create_test_project("PROJ-002", "Project 2");
    let forecast1 = create_test_forecast("PROJ-001", "Project 1");
    let forecast2 = create_test_forecast("PROJ-002", "Project 2");
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    ForecastRepositoryTrait::insert(&repo, forecast2);
    
    let project_service = ProjectService::new(&repo);
    let projects_without_forecasts = project_service.find_projects_without_forecasts();
    
    assert_eq!(projects_without_forecasts.len(), 0);
}

#[test]
fn test_project_service_find_projects_without_forecasts_none_have_forecasts() {
    let repo = InMemoryRepository::new();
    let project1 = create_test_project("PROJ-001", "Project 1");
    let project2 = create_test_project("PROJ-002", "Project 2");
    let project3 = create_test_project("PROJ-003", "Project 3");
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ProjectRepositoryTrait::insert(&repo, project3);
    
    let project_service = ProjectService::new(&repo);
    let projects_without_forecasts = project_service.find_projects_without_forecasts();
    
    assert_eq!(projects_without_forecasts.len(), 3);
}

#[test]
fn test_project_service_find_projects_without_forecasts_empty_repository() {
    let repo = InMemoryRepository::new();
    let project_service = ProjectService::new(&repo);
    let projects_without_forecasts = project_service.find_projects_without_forecasts();
    
    assert_eq!(projects_without_forecasts.len(), 0);
}

// Helper function to create a temporary projects CSV file
fn create_temp_projects_csv() -> PathBuf {
    let temp_dir = std::env::temp_dir();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let file_path = temp_dir.join(format!("test_projects_{}_{}.csv", std::process::id(), timestamp));
    
    // Remove file if it exists
    let _ = fs::remove_file(&file_path);
    
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "\"Project ID\",\"End Customer Name\",\"Project Name\",\"Manager\",\"Other Stakeholder\",\"Start\",\"Finish\",\"On Hold\",\"On Hold Comment\"").unwrap();
    writeln!(file, "\"code\",\"z_end_customer_name\",\"name\",\"manager\",\"z_other_stakeholder\",\"scheduleStart\",\"scheduleFinish\",\"z_ca_svcs_proj_hold\",\"z_ca_svcs_proj_holdcom\"").unwrap();
    writeln!(file, "\"PROJ-001\",\"Test Customer 1\",\"Test Project 1\",\"Manager 1\",\"AE 1\",\"2024-01-01\",\"2024-12-31\",\"false\",\"\"").unwrap();
    writeln!(file, "\"PROJ-002\",\"Test Customer 2\",\"Test Project 2\",\"Manager 2\",,\"2025-01-01\",\"2025-12-31\",\"true\",\"On hold for review\"").unwrap();
    writeln!(file, "\"PROJ-003\",\"Test Customer 3\",\"Test Project 3\",\"Manager 3\",\"AE 3\",\"2026-01-01\",\"2026-12-31\",\"false\",\"\"").unwrap();
    
    file_path
}

// Helper function to create a temporary forecasts CSV file
fn create_temp_forecasts_csv() -> PathBuf {
    let temp_dir = std::env::temp_dir();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let file_path = temp_dir.join(format!("test_forecasts_{}_{}.csv", std::process::id(), timestamp));
    
    // Remove file if it exists
    let _ = fs::remove_file(&file_path);
    
    let mut file = File::create(&file_path).unwrap();
    // First row: date headers
    writeln!(file, ",,,,,,,,,,\"November 01, 2025\",\"December 01, 2025\",\"January 01, 2026\",\"February 01, 2026\",\"March 01, 2026\",\"April 01, 2026\"").unwrap();
    // Second row: column headers
    writeln!(file, "Sold to Company,Project Manager,Project,ID,Class,Contract Start Date,Contract Finish Date,Contract Total Value,Contract Remaining Value,Currency,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ").unwrap();
    // Data rows
    writeln!(file, "Test Company 1,Manager 1,Test Project 1,PROJ-001,FP,01/01/2024,31/12/2024,100000.0,75000.0,USD,10000.0,15000.0,20000.0,15000.0,10000.0,5000.0").unwrap();
    writeln!(file, "Test Company 2,Manager 2,Test Project 2,PROJ-002,FP,01/01/2025,31/12/2025,200000.0,150000.0,USD,20000.0,25000.0,30000.0,25000.0,20000.0,10000.0").unwrap();
    writeln!(file, "Test Company 3,Manager 3,Test Project 3,PROJ-003,FP,01/01/2026,31/12/2026,300000.0,225000.0,USD,30000.0,35000.0,40000.0,35000.0,30000.0,15000.0").unwrap();
    
    file_path
}

// ========== CSV Loader Tests ==========

#[test]
fn test_load_projects_from_csv() {
    let repo = InMemoryRepository::new();
    let csv_path = create_temp_projects_csv();
    
    let result = load_projects_from_csv(&repo, csv_path.to_str().unwrap());
    assert!(result.is_ok());
    let count = result.unwrap();
    
    // Should load 3 projects (skipping the "code" header row)
    assert_eq!(count, 3);
    
    // Verify projects were loaded
    let projects = ProjectRepositoryTrait::find_all(&repo);
    assert_eq!(projects.len(), 3);
    
    // Verify first project
    let project1 = ProjectRepositoryTrait::find_by_id(&repo, "PROJ-001");
    assert!(project1.is_some());
    let p1 = project1.unwrap();
    assert_eq!(p1.project_id, "PROJ-001");
    assert_eq!(p1.project_name, "Test Project 1");
    assert_eq!(p1.end_customer_name, "Test Customer 1");
    assert_eq!(p1.on_hold, false);
    
    // Verify second project with on_hold = true
    let project2 = ProjectRepositoryTrait::find_by_id(&repo, "PROJ-002");
    assert!(project2.is_some());
    let p2 = project2.unwrap();
    assert_eq!(p2.on_hold, true);
    assert_eq!(p2.on_hold_comment, Some("On hold for review".to_string()));
    
    // Cleanup
    let _ = fs::remove_file(&csv_path);
}

#[test]
fn test_load_projects_from_csv_file_not_found() {
    let repo = InMemoryRepository::new();
    let result = load_projects_from_csv(&repo, "nonexistent_file.csv");
    assert!(result.is_err());
}

#[test]
fn test_load_projects_from_csv_empty_file() {
    let repo = InMemoryRepository::new();
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("empty_projects_{}.csv", std::process::id()));
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "\"Project ID\",\"End Customer Name\",\"Project Name\",\"Manager\",\"Other Stakeholder\",\"Start\",\"Finish\",\"On Hold\",\"On Hold Comment\"").unwrap();
    
    let result = load_projects_from_csv(&repo, file_path.to_str().unwrap());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
    
    let projects = ProjectRepositoryTrait::find_all(&repo);
    assert_eq!(projects.len(), 0);
    
    // Cleanup
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_load_forecasts_from_csv() {
    let repo = InMemoryRepository::new();
    let csv_path = create_temp_forecasts_csv();
    
    let result = load_forecasts_from_csv(&repo, csv_path.to_str().unwrap());
    assert!(result.is_ok());
    let count = result.unwrap();
    
    // Should load 3 forecasts
    assert_eq!(count, 3);
    
    // Verify forecasts were loaded
    let forecasts = ForecastRepositoryTrait::find_all(&repo);
    assert_eq!(forecasts.len(), 3);
    
    // Verify first forecast
    let forecast1 = ForecastRepositoryTrait::find_by_id(&repo, "PROJ-001");
    assert!(forecast1.is_some());
    let f1 = forecast1.unwrap();
    assert_eq!(f1.project_id, "PROJ-001");
    assert_eq!(f1.project_name, "Test Project 1");
    assert_eq!(f1.contract_total_value, 100000.0);
    assert_eq!(f1.contract_remaining_value, 75000.0);
    assert_eq!(f1.currency, "USD");
    assert_eq!(f1.month1_labor_revenue_commit, 10000.0);
    assert_eq!(f1.month6_labor_revenue_commit, 5000.0);
    
    // Verify second forecast
    let forecast2 = ForecastRepositoryTrait::find_by_id(&repo, "PROJ-002");
    assert!(forecast2.is_some());
    let f2 = forecast2.unwrap();
    assert_eq!(f2.contract_total_value, 200000.0);
    assert_eq!(f2.month2_labor_revenue_commit, 25000.0);
    
    // Cleanup
    let _ = fs::remove_file(&csv_path);
}

#[test]
fn test_load_forecasts_from_csv_file_not_found() {
    let repo = InMemoryRepository::new();
    let result = load_forecasts_from_csv(&repo, "nonexistent_file.csv");
    assert!(result.is_err());
}

#[test]
fn test_load_forecasts_from_csv_insufficient_columns() {
    let repo = InMemoryRepository::new();
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("invalid_forecasts_{}.csv", std::process::id()));
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, ",,,,,,,,,").unwrap();
    writeln!(file, "Sold to Company,Project Manager,Project,ID,Class,Contract Start Date,Contract Finish Date,Contract Total Value,Contract Remaining Value,Currency").unwrap();
    
    let result = load_forecasts_from_csv(&repo, file_path.to_str().unwrap());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Labor Rev Committ"));
    
    // Cleanup
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_load_forecasts_from_csv_empty_file() {
    let repo = InMemoryRepository::new();
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("empty_forecasts_{}.csv", std::process::id()));
    let _file = File::create(&file_path).unwrap();
    
    let result = load_forecasts_from_csv(&repo, file_path.to_str().unwrap());
    assert!(result.is_err()); // Should fail because it needs header row
    
    // Cleanup
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_load_data_from_csvs() {
    let repo = InMemoryRepository::new();
    let projects_path = create_temp_projects_csv();
    let forecasts_path = create_temp_forecasts_csv();
    
    let result = load_data_from_csvs(
        &repo,
        projects_path.to_str().unwrap(),
        forecasts_path.to_str().unwrap(),
    );
    assert!(result.is_ok());
    let (project_count, forecast_count) = result.unwrap();
    
    assert_eq!(project_count, 3);
    assert_eq!(forecast_count, 3);
    
    // Verify both repositories have data
    let projects = ProjectRepositoryTrait::find_all(&repo);
    let forecasts = ForecastRepositoryTrait::find_all(&repo);
    assert_eq!(projects.len(), 3);
    assert_eq!(forecasts.len(), 3);
    
    // Verify a project and forecast with same ID
    let project = ProjectRepositoryTrait::find_by_id(&repo, "PROJ-001");
    let forecast = ForecastRepositoryTrait::find_by_id(&repo, "PROJ-001");
    assert!(project.is_some());
    assert!(forecast.is_some());
    assert_eq!(project.unwrap().project_id, "PROJ-001");
    assert_eq!(forecast.unwrap().project_id, "PROJ-001");
    
    // Cleanup
    let _ = fs::remove_file(&projects_path);
    let _ = fs::remove_file(&forecasts_path);
}

#[test]
fn test_load_data_from_csvs_projects_error() {
    let repo = InMemoryRepository::new();
    let forecasts_path = create_temp_forecasts_csv();
    
    let result = load_data_from_csvs(
        &repo,
        "nonexistent_projects.csv",
        forecasts_path.to_str().unwrap(),
    );
    assert!(result.is_err());
    
    // Cleanup
    let _ = fs::remove_file(&forecasts_path);
}

#[test]
fn test_load_data_from_csvs_forecasts_error() {
    let repo = InMemoryRepository::new();
    let projects_path = create_temp_projects_csv();
    
    let result = load_data_from_csvs(
        &repo,
        projects_path.to_str().unwrap(),
        "nonexistent_forecasts.csv",
    );
    assert!(result.is_err());
    
    // Cleanup
    let _ = fs::remove_file(&projects_path);
}

#[test]
fn test_load_forecasts_from_csv_skips_empty_project_id() {
    let repo = InMemoryRepository::new();
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("forecasts_with_empty_{}.csv", std::process::id()));
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, ",,,,,,,,,,\"November 01, 2025\",\"December 01, 2025\",\"January 01, 2026\",\"February 01, 2026\",\"March 01, 2026\",\"April 01, 2026\"").unwrap();
    writeln!(file, "Sold to Company,Project Manager,Project,ID,Class,Contract Start Date,Contract Finish Date,Contract Total Value,Contract Remaining Value,Currency,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ").unwrap();
    writeln!(file, "Test Company,Manager,Test Project,PROJ-001,FP,01/01/2024,31/12/2024,100000.0,75000.0,USD,10000.0,15000.0,20000.0,15000.0,10000.0,5000.0").unwrap();
    writeln!(file, ",,,,,,,,,,,,,").unwrap(); // Empty row
    writeln!(file, "Test Company 2,Manager 2,Test Project 2,PROJ-002,FP,01/01/2025,31/12/2025,200000.0,150000.0,USD,20000.0,25000.0,30000.0,25000.0,20000.0,10000.0").unwrap();
    
    let result = load_forecasts_from_csv(&repo, file_path.to_str().unwrap());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2); // Should skip empty row
    
    let forecasts = ForecastRepositoryTrait::find_all(&repo);
    assert_eq!(forecasts.len(), 2);
    
    // Cleanup
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_load_forecasts_from_csv_handles_empty_numeric_fields() {
    let repo = InMemoryRepository::new();
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("forecasts_empty_numeric_{}.csv", std::process::id()));
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, ",,,,,,,,,,\"November 01, 2025\",\"December 01, 2025\",\"January 01, 2026\",\"February 01, 2026\",\"March 01, 2026\",\"April 01, 2026\"").unwrap();
    writeln!(file, "Sold to Company,Project Manager,Project,ID,Class,Contract Start Date,Contract Finish Date,Contract Total Value,Contract Remaining Value,Currency,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ").unwrap();
    writeln!(file, "Test Company,Manager,Test Project,PROJ-001,FP,01/01/2024,31/12/2024,,,USD,,,20000.0,15000.0,10000.0,").unwrap();
    
    let result = load_forecasts_from_csv(&repo, file_path.to_str().unwrap());
    assert!(result.is_ok());
    
    let forecast = ForecastRepositoryTrait::find_by_id(&repo, "PROJ-001");
    assert!(forecast.is_some());
    let f = forecast.unwrap();
    // Empty numeric fields should default to 0.0
    assert_eq!(f.contract_total_value, 0.0);
    assert_eq!(f.contract_remaining_value, 0.0);
    assert_eq!(f.month1_labor_revenue_commit, 0.0);
    assert_eq!(f.month3_labor_revenue_commit, 20000.0);
    assert_eq!(f.month6_labor_revenue_commit, 0.0);
    
    // Cleanup
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_load_projects_from_csv_with_optional_fields() {
    let repo = InMemoryRepository::new();
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("projects_optional_{}.csv", std::process::id()));
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "\"Project ID\",\"End Customer Name\",\"Project Name\",\"Manager\",\"Other Stakeholder\",\"Start\",\"Finish\",\"On Hold\",\"On Hold Comment\"").unwrap();
    writeln!(file, "\"code\",\"z_end_customer_name\",\"name\",\"manager\",\"z_other_stakeholder\",\"scheduleStart\",\"scheduleFinish\",\"z_ca_svcs_proj_hold\",\"z_ca_svcs_proj_holdcom\"").unwrap();
    writeln!(file, "\"PROJ-001\",\"Customer 1\",\"Project 1\",\"Manager 1\",,\"2024-01-01\",\"2024-12-31\",\"false\",\"\"").unwrap();
    writeln!(file, "\"PROJ-002\",\"Customer 2\",\"Project 2\",\"Manager 2\",\"AE 2\",\"2025-01-01\",\"2025-12-31\",\"True\",\"Comment\"").unwrap();
    
    let result = load_projects_from_csv(&repo, file_path.to_str().unwrap());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2);
    
    // Verify project without optional field
    let project1 = ProjectRepositoryTrait::find_by_id(&repo, "PROJ-001");
    assert!(project1.is_some());
    assert_eq!(project1.unwrap().account_executive, None);
    
    // Verify project with optional field
    let project2 = ProjectRepositoryTrait::find_by_id(&repo, "PROJ-002");
    assert!(project2.is_some());
    assert_eq!(project2.unwrap().account_executive, Some("AE 2".to_string()));
    
    // Cleanup
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_load_projects_from_csv_updates_existing() {
    let repo = InMemoryRepository::new();
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("projects_duplicate_{}.csv", std::process::id()));
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "\"Project ID\",\"End Customer Name\",\"Project Name\",\"Manager\",\"Other Stakeholder\",\"Start\",\"Finish\",\"On Hold\",\"On Hold Comment\"").unwrap();
    writeln!(file, "\"code\",\"z_end_customer_name\",\"name\",\"manager\",\"z_other_stakeholder\",\"scheduleStart\",\"scheduleFinish\",\"z_ca_svcs_proj_hold\",\"z_ca_svcs_proj_holdcom\"").unwrap();
    writeln!(file, "\"PROJ-001\",\"Original Customer\",\"Original Project\",\"Manager 1\",,\"2024-01-01\",\"2024-12-31\",\"false\",\"\"").unwrap();
    writeln!(file, "\"PROJ-001\",\"Updated Customer\",\"Updated Project\",\"Manager 2\",\"AE 2\",\"2025-01-01\",\"2025-12-31\",\"true\",\"Updated comment\"").unwrap();
    
    let result = load_projects_from_csv(&repo, file_path.to_str().unwrap());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2);
    
    // Should only have one project (last one overwrites)
    let projects = ProjectRepositoryTrait::find_all(&repo);
    assert_eq!(projects.len(), 1);
    
    // Verify the updated project
    let project = ProjectRepositoryTrait::find_by_id(&repo, "PROJ-001");
    assert!(project.is_some());
    let p = project.unwrap();
    assert_eq!(p.project_name, "Updated Project");
    assert_eq!(p.end_customer_name, "Updated Customer");
    assert_eq!(p.on_hold, true);
    assert_eq!(p.on_hold_comment, Some("Updated comment".to_string()));
    
    // Cleanup
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_load_projects_from_csv_skips_code_header() {
    let repo = InMemoryRepository::new();
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("projects_header_{}.csv", std::process::id()));
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "\"Project ID\",\"End Customer Name\",\"Project Name\",\"Manager\",\"Other Stakeholder\",\"Start\",\"Finish\",\"On Hold\",\"On Hold Comment\"").unwrap();
    writeln!(file, "\"code\",\"z_end_customer_name\",\"name\",\"manager\",\"z_other_stakeholder\",\"scheduleStart\",\"scheduleFinish\",\"z_ca_svcs_proj_hold\",\"z_ca_svcs_proj_holdcom\"").unwrap();
    writeln!(file, "\"PROJ-001\",\"Customer 1\",\"Project 1\",\"Manager 1\",,\"2024-01-01\",\"2024-12-31\",\"false\",\"\"").unwrap();
    
    let result = load_projects_from_csv(&repo, file_path.to_str().unwrap());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
    
    // Should skip the "code" row and only load the actual project
    let projects = ProjectRepositoryTrait::find_all(&repo);
    assert_eq!(projects.len(), 1);
    
    // Should not have a project with ID "code"
    let code_project = ProjectRepositoryTrait::find_by_id(&repo, "code");
    assert!(code_project.is_none());
    
    // Cleanup
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_load_forecasts_from_csv_missing_header_row() {
    let repo = InMemoryRepository::new();
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("forecasts_no_header_{}.csv", std::process::id()));
    let mut file = File::create(&file_path).unwrap();
    // Only date row, missing actual header row
    writeln!(file, ",,,,,,,,,,\"November 01, 2025\",\"December 01, 2025\",\"January 01, 2026\",\"February 01, 2026\",\"March 01, 2026\",\"April 01, 2026\"").unwrap();
    
    let result = load_forecasts_from_csv(&repo, file_path.to_str().unwrap());
    assert!(result.is_err());
    
    // Cleanup
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_load_forecasts_from_csv_skips_id_header_row() {
    let repo = InMemoryRepository::new();
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("forecasts_id_header_{}.csv", std::process::id()));
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, ",,,,,,,,,,\"November 01, 2025\",\"December 01, 2025\",\"January 01, 2026\",\"February 01, 2026\",\"March 01, 2026\",\"April 01, 2026\"").unwrap();
    writeln!(file, "Sold to Company,Project Manager,Project,ID,Class,Contract Start Date,Contract Finish Date,Contract Total Value,Contract Remaining Value,Currency,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ").unwrap();
    writeln!(file, "Test Company,Manager,Test Project,PROJ-001,FP,01/01/2024,31/12/2024,100000.0,75000.0,USD,10000.0,15000.0,20000.0,15000.0,10000.0,5000.0").unwrap();
    
    let result = load_forecasts_from_csv(&repo, file_path.to_str().unwrap());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
    
    // Verify forecast was loaded
    let forecast = ForecastRepositoryTrait::find_by_id(&repo, "PROJ-001");
    assert!(forecast.is_some());
    
    // Cleanup
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_load_forecasts_from_csv_updates_existing() {
    let repo = InMemoryRepository::new();
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("forecasts_duplicate_{}.csv", std::process::id()));
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, ",,,,,,,,,,\"November 01, 2025\",\"December 01, 2025\",\"January 01, 2026\",\"February 01, 2026\",\"March 01, 2026\",\"April 01, 2026\"").unwrap();
    writeln!(file, "Sold to Company,Project Manager,Project,ID,Class,Contract Start Date,Contract Finish Date,Contract Total Value,Contract Remaining Value,Currency,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ").unwrap();
    writeln!(file, "Original Company,Original Manager,Original Project,PROJ-001,FP,01/01/2024,31/12/2024,100000.0,75000.0,USD,10000.0,15000.0,20000.0,15000.0,10000.0,5000.0").unwrap();
    writeln!(file, "Updated Company,Updated Manager,Updated Project,PROJ-001,FP,01/01/2025,31/12/2025,200000.0,150000.0,EUR,20000.0,25000.0,30000.0,25000.0,20000.0,10000.0").unwrap();
    
    let result = load_forecasts_from_csv(&repo, file_path.to_str().unwrap());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2);
    
    // Should only have one forecast (last one overwrites)
    let forecasts = ForecastRepositoryTrait::find_all(&repo);
    assert_eq!(forecasts.len(), 1);
    
    // Verify the updated forecast
    let forecast = ForecastRepositoryTrait::find_by_id(&repo, "PROJ-001");
    assert!(forecast.is_some());
    let f = forecast.unwrap();
    assert_eq!(f.project_name, "Updated Project");
    assert_eq!(f.sold_to_company, "Updated Company");
    assert_eq!(f.contract_total_value, 200000.0);
    assert_eq!(f.currency, "EUR");
    
    // Cleanup
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_load_data_from_csvs_updates_existing_data() {
    let repo = InMemoryRepository::new();
    
    // First load
    let projects_path = create_temp_projects_csv();
    let forecasts_path = create_temp_forecasts_csv();
    let result1 = load_data_from_csvs(
        &repo,
        projects_path.to_str().unwrap(),
        forecasts_path.to_str().unwrap(),
    );
    assert!(result1.is_ok());
    
    let projects1 = ProjectRepositoryTrait::find_all(&repo);
    let forecasts1 = ForecastRepositoryTrait::find_all(&repo);
    assert_eq!(projects1.len(), 3);
    assert_eq!(forecasts1.len(), 3);
    
    // Second load with same IDs should update
    let result2 = load_data_from_csvs(
        &repo,
        projects_path.to_str().unwrap(),
        forecasts_path.to_str().unwrap(),
    );
    assert!(result2.is_ok());
    
    // Should still have same count (updated, not duplicated)
    let projects2 = ProjectRepositoryTrait::find_all(&repo);
    let forecasts2 = ForecastRepositoryTrait::find_all(&repo);
    assert_eq!(projects2.len(), 3);
    assert_eq!(forecasts2.len(), 3);
    
    // Cleanup
    let _ = fs::remove_file(&projects_path);
    let _ = fs::remove_file(&forecasts_path);
}


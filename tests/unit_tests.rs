use rev_report::db::repositories::forecast_repository::{ForecastRepository, ForecastRepositoryTrait};
use rev_report::db::repositories::in_memory_repository::InMemoryRepository;
use rev_report::db::repositories::project_repository::{ProjectRepository, ProjectRepositoryTrait};
use rev_report::models::forecast::Forecast;
use rev_report::models::project::Project;

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


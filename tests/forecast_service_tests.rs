mod common;

use common::*;
use rev_report::db::repositories::forecast_repository::ForecastRepositoryTrait;
use rev_report::db::repositories::in_memory_repository::InMemoryRepository;
use rev_report::db::repositories::project_repository::ProjectRepositoryTrait;
use rev_report::services::forecast_service::ForecastService;

#[test]
fn test_forecast_service_get_forecasts_by_project_manager_empty_repository() {
    let repo = InMemoryRepository::new();
    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_forecasts_by_project_manager();
    
    assert_eq!(result.len(), 0);
}

#[test]
fn test_forecast_service_get_forecasts_by_project_manager_single_manager() {
    let repo = InMemoryRepository::new();
    let project1 = create_test_project("PROJ-001", "Project 1");
    let project2 = create_test_project("PROJ-002", "Project 2");
    let mut forecast1 = create_test_forecast("PROJ-001", "Project 1");
    forecast1.project_manager = "Smith, John".to_string();
    let mut forecast2 = create_test_forecast("PROJ-002", "Project 2");
    forecast2.project_manager = "Smith, John".to_string();
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    ForecastRepositoryTrait::insert(&repo, forecast2);
    
    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_forecasts_by_project_manager();
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].project_manager, "Smith, John");
    assert_eq!(result[0].forecasts.len(), 2);
}

#[test]
fn test_forecast_service_get_forecasts_by_project_manager_multiple_managers() {
    let repo = InMemoryRepository::new();
    let project1 = create_test_project("PROJ-001", "Project 1");
    let project2 = create_test_project("PROJ-002", "Project 2");
    let project3 = create_test_project("PROJ-003", "Project 3");
    let mut forecast1 = create_test_forecast("PROJ-001", "Project 1");
    forecast1.project_manager = "Smith, John".to_string();
    let mut forecast2 = create_test_forecast("PROJ-002", "Project 2");
    forecast2.project_manager = "Doe, Jane".to_string();
    let mut forecast3 = create_test_forecast("PROJ-003", "Project 3");
    forecast3.project_manager = "Smith, John".to_string();
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ProjectRepositoryTrait::insert(&repo, project3);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    ForecastRepositoryTrait::insert(&repo, forecast2);
    ForecastRepositoryTrait::insert(&repo, forecast3);
    
    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_forecasts_by_project_manager();
    
    assert_eq!(result.len(), 2);
    // Should be sorted alphabetically
    assert_eq!(result[0].project_manager, "Doe, Jane");
    assert_eq!(result[0].forecasts.len(), 1);
    assert_eq!(result[1].project_manager, "Smith, John");
    assert_eq!(result[1].forecasts.len(), 2);
}

#[test]
fn test_forecast_service_get_forecasts_by_project_manager_with_account_id() {
    let repo = InMemoryRepository::new();
    let project1 = create_test_project("PROJ-001", "Project 1");
    let mut forecast1 = create_test_forecast("PROJ-001", "Project 1");
    forecast1.project_manager = "Smith, John{js055528@broadcom.net}".to_string();
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    
    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_forecasts_by_project_manager();
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].project_manager, "Smith, John");
    assert_eq!(result[0].forecasts.len(), 1);
}

#[test]
fn test_forecast_service_get_forecasts_by_project_manager_without_project() {
    let repo = InMemoryRepository::new();
    let mut forecast1 = create_test_forecast("PROJ-001", "Project 1");
    forecast1.project_manager = "Smith, John".to_string();
    
    // Don't insert the project, so customer name should be None
    ForecastRepositoryTrait::insert(&repo, forecast1);
    
    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_forecasts_by_project_manager();
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].project_manager, "Smith, John");
    assert_eq!(result[0].forecasts.len(), 1);
    // Customer name should be None since project doesn't exist
    assert!(result[0].forecasts[0].1.is_none());
}

#[test]
fn test_forecast_service_get_forecasts_by_project_manager_with_customer_name() {
    let repo = InMemoryRepository::new();
    let project1 = create_test_project("PROJ-001", "Project 1");
    let mut forecast1 = create_test_forecast("PROJ-001", "Project 1");
    forecast1.project_manager = "Smith, John".to_string();
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    
    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_forecasts_by_project_manager();
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].forecasts.len(), 1);
    // Customer name should be Some since project exists
    assert!(result[0].forecasts[0].1.is_some());
    assert_eq!(result[0].forecasts[0].1.as_ref().unwrap(), "Test Customer");
}

#[test]
fn test_forecast_service_get_forecasts_by_project_manager_sorting() {
    let repo = InMemoryRepository::new();
    let project1 = create_test_project("PROJ-001", "Project 1");
    let project2 = create_test_project("PROJ-002", "Project 2");
    let project3 = create_test_project("PROJ-003", "Project 3");
    let mut forecast1 = create_test_forecast("PROJ-001", "Project 1");
    forecast1.project_manager = "Zebra, Zoe".to_string();
    let mut forecast2 = create_test_forecast("PROJ-002", "Project 2");
    forecast2.project_manager = "Apple, Adam".to_string();
    let mut forecast3 = create_test_forecast("PROJ-003", "Project 3");
    forecast3.project_manager = "Baker, Bob".to_string();
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ProjectRepositoryTrait::insert(&repo, project3);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    ForecastRepositoryTrait::insert(&repo, forecast2);
    ForecastRepositoryTrait::insert(&repo, forecast3);
    
    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_forecasts_by_project_manager();
    
    assert_eq!(result.len(), 3);
    // Should be sorted alphabetically
    assert_eq!(result[0].project_manager, "Apple, Adam");
    assert_eq!(result[1].project_manager, "Baker, Bob");
    assert_eq!(result[2].project_manager, "Zebra, Zoe");
}


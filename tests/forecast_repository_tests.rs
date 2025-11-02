mod common;

use common::*;
use rev_report::db::repositories::forecast_repository::{ForecastRepository, ForecastRepositoryTrait};

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


mod common;

use common::*;
use rev_report::db::repositories::forecast_repository::ForecastRepositoryTrait;
use rev_report::db::repositories::in_memory_repository::InMemoryRepository;
use rev_report::db::repositories::project_repository::ProjectRepositoryTrait;

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

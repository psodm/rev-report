mod common;

use common::*;
use rev_report::db::repositories::forecast_repository::ForecastRepositoryTrait;
use rev_report::db::repositories::in_memory_repository::InMemoryRepository;
use rev_report::db::repositories::project_repository::ProjectRepositoryTrait;
use rev_report::services::project_service::ProjectService;

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

#[test]
fn test_project_service_find_projects_without_forecasts_with_zero_forecast() {
    let repo = InMemoryRepository::new();
    let project1 = create_test_project("PROJ-001", "Project 1");
    let project2 = create_test_project("PROJ-002", "Project 2");
    let mut forecast1 = create_test_forecast("PROJ-001", "Project 1");
    // Set all month values to 0
    forecast1.month1_labor_revenue_commit = 0.0;
    forecast1.month2_labor_revenue_commit = 0.0;
    forecast1.month3_labor_revenue_commit = 0.0;
    forecast1.month4_labor_revenue_commit = 0.0;
    forecast1.month5_labor_revenue_commit = 0.0;
    forecast1.month6_labor_revenue_commit = 0.0;
    // PROJ-002 has no forecast
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    
    let project_service = ProjectService::new(&repo);
    let projects_without_forecasts = project_service.find_projects_without_forecasts();
    
    // Both projects should be in the list: PROJ-001 has zero forecast, PROJ-002 has no forecast
    assert_eq!(projects_without_forecasts.len(), 2);
    let project_ids: Vec<String> = projects_without_forecasts.iter().map(|p| p.project_id.clone()).collect();
    assert!(project_ids.contains(&"PROJ-001".to_string()));
    assert!(project_ids.contains(&"PROJ-002".to_string()));
}

#[test]
fn test_project_service_find_projects_without_forecasts_with_partial_zero_forecast() {
    let repo = InMemoryRepository::new();
    let project1 = create_test_project("PROJ-001", "Project 1");
    let project2 = create_test_project("PROJ-002", "Project 2");
    let mut forecast1 = create_test_forecast("PROJ-001", "Project 1");
    // Set most month values to 0, but one is non-zero
    forecast1.month1_labor_revenue_commit = 0.0;
    forecast1.month2_labor_revenue_commit = 0.0;
    forecast1.month3_labor_revenue_commit = 1000.0; // Non-zero value
    forecast1.month4_labor_revenue_commit = 0.0;
    forecast1.month5_labor_revenue_commit = 0.0;
    forecast1.month6_labor_revenue_commit = 0.0;
    let forecast2 = create_test_forecast("PROJ-002", "Project 2");
    // PROJ-002 has a forecast with non-zero values
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    ForecastRepositoryTrait::insert(&repo, forecast2);
    
    let project_service = ProjectService::new(&repo);
    let projects_without_forecasts = project_service.find_projects_without_forecasts();
    
    // Neither project should be in the list: both have non-zero forecasts
    assert_eq!(projects_without_forecasts.len(), 0);
}

#[test]
fn test_project_service_find_projects_without_forecasts_mixed_scenarios() {
    let repo = InMemoryRepository::new();
    let project1 = create_test_project("PROJ-001", "Project 1"); // No forecast
    let project2 = create_test_project("PROJ-002", "Project 2"); // Zero forecast
    let project3 = create_test_project("PROJ-003", "Project 3"); // Non-zero forecast
    let project4 = create_test_project("PROJ-004", "Project 4"); // No forecast
    let mut forecast2 = create_test_forecast("PROJ-002", "Project 2");
    // Set all month values to 0
    forecast2.month1_labor_revenue_commit = 0.0;
    forecast2.month2_labor_revenue_commit = 0.0;
    forecast2.month3_labor_revenue_commit = 0.0;
    forecast2.month4_labor_revenue_commit = 0.0;
    forecast2.month5_labor_revenue_commit = 0.0;
    forecast2.month6_labor_revenue_commit = 0.0;
    let forecast3 = create_test_forecast("PROJ-003", "Project 3");
    // PROJ-003 has a forecast with non-zero values (from create_test_forecast)
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ProjectRepositoryTrait::insert(&repo, project3);
    ProjectRepositoryTrait::insert(&repo, project4);
    ForecastRepositoryTrait::insert(&repo, forecast2);
    ForecastRepositoryTrait::insert(&repo, forecast3);
    
    let project_service = ProjectService::new(&repo);
    let projects_without_forecasts = project_service.find_projects_without_forecasts();
    
    // PROJ-001 and PROJ-004 have no forecast, PROJ-002 has zero forecast
    // PROJ-003 has non-zero forecast, so it should not be in the list
    assert_eq!(projects_without_forecasts.len(), 3);
    let project_ids: Vec<String> = projects_without_forecasts.iter().map(|p| p.project_id.clone()).collect();
    assert!(project_ids.contains(&"PROJ-001".to_string()));
    assert!(project_ids.contains(&"PROJ-002".to_string()));
    assert!(project_ids.contains(&"PROJ-004".to_string()));
    assert!(!project_ids.contains(&"PROJ-003".to_string()));
}

#[test]
fn test_project_service_find_projects_without_account_executive() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.account_executive = None; // No account executive
    let mut project2 = create_test_project("PROJ-002", "Project 2");
    project2.account_executive = Some("".to_string()); // Empty string
    let mut project3 = create_test_project("PROJ-003", "Project 3");
    project3.account_executive = Some("   ".to_string()); // Whitespace only
    let project4 = create_test_project("PROJ-004", "Project 4");
    // Has account executive (default from create_test_project)
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ProjectRepositoryTrait::insert(&repo, project3);
    ProjectRepositoryTrait::insert(&repo, project4);
    
    let project_service = ProjectService::new(&repo);
    let projects_without_ae = project_service.find_projects_without_account_executive();
    
    // PROJ-001, PROJ-002, and PROJ-003 should be in the list
    // PROJ-004 has an account executive, so it should not be in the list
    assert_eq!(projects_without_ae.len(), 3);
    let project_ids: Vec<String> = projects_without_ae.iter().map(|p| p.project_id.clone()).collect();
    assert!(project_ids.contains(&"PROJ-001".to_string()));
    assert!(project_ids.contains(&"PROJ-002".to_string()));
    assert!(project_ids.contains(&"PROJ-003".to_string()));
    assert!(!project_ids.contains(&"PROJ-004".to_string()));
}

#[test]
fn test_project_service_find_projects_without_account_executive_all_have_ae() {
    let repo = InMemoryRepository::new();
    let project1 = create_test_project("PROJ-001", "Project 1");
    let project2 = create_test_project("PROJ-002", "Project 2");
    // Both have account executives (from create_test_project)
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    
    let project_service = ProjectService::new(&repo);
    let projects_without_ae = project_service.find_projects_without_account_executive();
    
    assert_eq!(projects_without_ae.len(), 0);
}

#[test]
fn test_project_service_find_projects_without_account_executive_none_have_ae() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.account_executive = None;
    let mut project2 = create_test_project("PROJ-002", "Project 2");
    project2.account_executive = None;
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    
    let project_service = ProjectService::new(&repo);
    let projects_without_ae = project_service.find_projects_without_account_executive();
    
    assert_eq!(projects_without_ae.len(), 2);
}

#[test]
fn test_project_service_find_projects_without_account_executive_empty_repository() {
    let repo = InMemoryRepository::new();
    let project_service = ProjectService::new(&repo);
    let projects_without_ae = project_service.find_projects_without_account_executive();
    
    assert_eq!(projects_without_ae.len(), 0);
}

#[test]
fn test_project_service_find_all_account_executives() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.account_executive = Some("John Doe".to_string());
    let mut project2 = create_test_project("PROJ-002", "Project 2");
    project2.account_executive = Some("Jane Smith".to_string());
    let mut project3 = create_test_project("PROJ-003", "Project 3");
    project3.account_executive = Some("John Doe".to_string()); // Duplicate
    let mut project4 = create_test_project("PROJ-004", "Project 4");
    project4.account_executive = Some("Bob Johnson".to_string());
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ProjectRepositoryTrait::insert(&repo, project3);
    ProjectRepositoryTrait::insert(&repo, project4);
    
    let project_service = ProjectService::new(&repo);
    let account_executives = project_service.find_all_account_executives();
    
    // Should have 3 unique account executives, sorted alphabetically
    assert_eq!(account_executives.len(), 3);
    assert_eq!(account_executives[0], "Bob Johnson");
    assert_eq!(account_executives[1], "Jane Smith");
    assert_eq!(account_executives[2], "John Doe");
}

#[test]
fn test_project_service_find_all_account_executives_empty_repository() {
    let repo = InMemoryRepository::new();
    let project_service = ProjectService::new(&repo);
    let account_executives = project_service.find_all_account_executives();
    
    assert_eq!(account_executives.len(), 0);
}

#[test]
fn test_project_service_find_all_account_executives_no_account_executives() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.account_executive = None;
    let mut project2 = create_test_project("PROJ-002", "Project 2");
    project2.account_executive = Some("".to_string()); // Empty string
    let mut project3 = create_test_project("PROJ-003", "Project 3");
    project3.account_executive = Some("   ".to_string()); // Whitespace only
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ProjectRepositoryTrait::insert(&repo, project3);
    
    let project_service = ProjectService::new(&repo);
    let account_executives = project_service.find_all_account_executives();
    
    // Should have 0 account executives (empty/whitespace-only are ignored)
    assert_eq!(account_executives.len(), 0);
}

#[test]
fn test_project_service_find_all_account_executives_with_whitespace() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.account_executive = Some("  John Doe  ".to_string()); // With whitespace
    let mut project2 = create_test_project("PROJ-002", "Project 2");
    project2.account_executive = Some("John Doe".to_string()); // Without whitespace
    let mut project3 = create_test_project("PROJ-003", "Project 3");
    project3.account_executive = Some("Jane Smith".to_string());
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ProjectRepositoryTrait::insert(&repo, project3);
    
    let project_service = ProjectService::new(&repo);
    let account_executives = project_service.find_all_account_executives();
    
    // Should have 2 unique account executives (trimmed "John Doe" should be deduplicated)
    // "  John Doe  " trimmed becomes "John Doe", so it should match the other "John Doe"
    assert_eq!(account_executives.len(), 2);
    assert_eq!(account_executives[0], "Jane Smith");
    assert_eq!(account_executives[1], "John Doe");
}

#[test]
fn test_project_service_get_projects_by_project_manager_empty_repository() {
    let repo = InMemoryRepository::new();
    let project_service = ProjectService::new(&repo);
    let result = project_service.get_projects_by_project_manager();
    
    assert_eq!(result.len(), 0);
}

#[test]
fn test_project_service_get_projects_by_project_manager_single_manager() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.project_manager = "Smith, John".to_string();
    let mut project2 = create_test_project("PROJ-002", "Project 2");
    project2.project_manager = "Smith, John".to_string();
    let forecast1 = create_test_forecast("PROJ-001", "Project 1");
    let forecast2 = create_test_forecast("PROJ-002", "Project 2");
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    ForecastRepositoryTrait::insert(&repo, forecast2);
    
    let project_service = ProjectService::new(&repo);
    let result = project_service.get_projects_by_project_manager();
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].project_manager, "Smith, John");
    assert_eq!(result[0].projects.len(), 2);
    
    // Check that contract values are included
    let (_, contract_values1) = &result[0].projects[0];
    assert!(contract_values1.is_some());
    let (total1, remaining1, currency1, class1) = contract_values1.as_ref().unwrap();
    assert_eq!(*total1, 100000.0);
    assert_eq!(*remaining1, 75000.0);
    assert_eq!(currency1, "USD");
    assert_eq!(class1, "Test Class");
}

#[test]
fn test_project_service_get_projects_by_project_manager_multiple_managers() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.project_manager = "Smith, John".to_string();
    let mut project2 = create_test_project("PROJ-002", "Project 2");
    project2.project_manager = "Doe, Jane".to_string();
    let mut project3 = create_test_project("PROJ-003", "Project 3");
    project3.project_manager = "Smith, John".to_string();
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ProjectRepositoryTrait::insert(&repo, project3);
    
    let project_service = ProjectService::new(&repo);
    let result = project_service.get_projects_by_project_manager();
    
    // Should be sorted alphabetically: "Doe, Jane" comes before "Smith, John"
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].project_manager, "Doe, Jane");
    assert_eq!(result[0].projects.len(), 1);
    assert_eq!(result[1].project_manager, "Smith, John");
    assert_eq!(result[1].projects.len(), 2);
}

#[test]
fn test_project_service_get_projects_by_project_manager_extracts_manager_name() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.project_manager = "Smith, John{jsmith@example.com}".to_string();
    let mut project2 = create_test_project("PROJ-002", "Project 2");
    project2.project_manager = "Smith, John{jsmith@example.com}".to_string();
    let mut project3 = create_test_project("PROJ-003", "Project 3");
    project3.project_manager = "Doe, Jane{jdoe@example.com}".to_string();
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ProjectRepositoryTrait::insert(&repo, project3);
    
    let project_service = ProjectService::new(&repo);
    let result = project_service.get_projects_by_project_manager();
    
    // Should extract just the name part (before the '{')
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].project_manager, "Doe, Jane");
    assert_eq!(result[1].project_manager, "Smith, John");
}

#[test]
fn test_project_service_get_projects_by_project_manager_without_forecasts() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.project_manager = "Smith, John".to_string();
    let mut project2 = create_test_project("PROJ-002", "Project 2");
    project2.project_manager = "Smith, John".to_string();
    let forecast1 = create_test_forecast("PROJ-001", "Project 1");
    // PROJ-002 has no forecast
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    
    let project_service = ProjectService::new(&repo);
    let result = project_service.get_projects_by_project_manager();
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].projects.len(), 2);
    
    // Find projects by ID (order is not guaranteed with HashMap)
    let (proj1_item, contract_values1) = result[0]
        .projects
        .iter()
        .find(|(p, _)| p.project_id == "PROJ-001")
        .unwrap();
    assert_eq!(proj1_item.project_id, "PROJ-001");
    assert!(contract_values1.is_some());
    
    let (proj2_item, contract_values2) = result[0]
        .projects
        .iter()
        .find(|(p, _)| p.project_id == "PROJ-002")
        .unwrap();
    assert_eq!(proj2_item.project_id, "PROJ-002");
    assert!(contract_values2.is_none());
}

#[test]
fn test_project_service_get_projects_by_project_manager_with_different_currencies() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.project_manager = "Smith, John".to_string();
    let mut project2 = create_test_project("PROJ-002", "Project 2");
    project2.project_manager = "Smith, John".to_string();
    let mut forecast1 = create_test_forecast("PROJ-001", "Project 1");
    forecast1.currency = "EUR".to_string();
    forecast1.contract_total_value = 50000.0;
    forecast1.contract_remaining_value = 37500.0;
    let mut forecast2 = create_test_forecast("PROJ-002", "Project 2");
    forecast2.currency = "GBP".to_string();
    forecast2.contract_total_value = 75000.0;
    forecast2.contract_remaining_value = 56250.0;
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    ForecastRepositoryTrait::insert(&repo, forecast2);
    
    let project_service = ProjectService::new(&repo);
    let result = project_service.get_projects_by_project_manager();
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].projects.len(), 2);
    
    // Find projects by ID (order is not guaranteed with HashMap)
    let (proj1_item, contract_values1) = result[0]
        .projects
        .iter()
        .find(|(p, _)| p.project_id == "PROJ-001")
        .unwrap();
    assert_eq!(proj1_item.project_id, "PROJ-001");
    let (total1, remaining1, currency1, class1) = contract_values1.as_ref().unwrap();
    assert_eq!(*total1, 50000.0);
    assert_eq!(*remaining1, 37500.0);
    assert_eq!(currency1, "EUR");
    assert_eq!(class1, "Test Class");
    
    let (proj2_item, contract_values2) = result[0]
        .projects
        .iter()
        .find(|(p, _)| p.project_id == "PROJ-002")
        .unwrap();
    assert_eq!(proj2_item.project_id, "PROJ-002");
    let (total2, remaining2, currency2, class2) = contract_values2.as_ref().unwrap();
    assert_eq!(*total2, 75000.0);
    assert_eq!(*remaining2, 56250.0);
    assert_eq!(currency2, "GBP");
    assert_eq!(class2, "Test Class");
}

#[test]
fn test_project_service_get_projects_by_project_manager_sorted_alphabetically() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.project_manager = "Zebra, Alice".to_string();
    let mut project2 = create_test_project("PROJ-002", "Project 2");
    project2.project_manager = "Apple, Bob".to_string();
    let mut project3 = create_test_project("PROJ-003", "Project 3");
    project3.project_manager = "Miller, Charlie".to_string();
    
    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ProjectRepositoryTrait::insert(&repo, project3);
    
    let project_service = ProjectService::new(&repo);
    let result = project_service.get_projects_by_project_manager();
    
    // Should be sorted alphabetically
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].project_manager, "Apple, Bob");
    assert_eq!(result[1].project_manager, "Miller, Charlie");
    assert_eq!(result[2].project_manager, "Zebra, Alice");
}


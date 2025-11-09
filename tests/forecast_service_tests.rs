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

#[test]
fn test_forecast_service_get_revenue_summary_by_account_executive_empty_repository() {
    let repo = InMemoryRepository::new();
    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_revenue_summary_by_account_executive();

    assert_eq!(result.len(), 0);
}

#[test]
fn test_forecast_service_get_revenue_summary_by_account_executive_single_ae_single_project() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.account_executive = Some("AE One".to_string());
    let mut forecast1 = create_test_forecast("PROJ-001", "Project 1");
    forecast1.month1_labor_revenue_commit = 10000.0;
    forecast1.month2_labor_revenue_commit = 20000.0;
    forecast1.month3_labor_revenue_commit = 30000.0;
    forecast1.month4_labor_revenue_commit = 40000.0;
    forecast1.month5_labor_revenue_commit = 50000.0;
    forecast1.month6_labor_revenue_commit = 60000.0;

    ProjectRepositoryTrait::insert(&repo, project1);
    ForecastRepositoryTrait::insert(&repo, forecast1);

    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_revenue_summary_by_account_executive();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].account_executive, "AE One");
    assert_eq!(result[0].month1_total, 10000.0);
    assert_eq!(result[0].month2_total, 20000.0);
    assert_eq!(result[0].month3_total, 30000.0);
    assert_eq!(result[0].month4_total, 40000.0);
    assert_eq!(result[0].month5_total, 50000.0);
    assert_eq!(result[0].month6_total, 60000.0);
}

#[test]
fn test_forecast_service_get_revenue_summary_by_account_executive_single_ae_multiple_projects() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.account_executive = Some("AE One".to_string());
    let mut project2 = create_test_project("PROJ-002", "Project 2");
    project2.account_executive = Some("AE One".to_string());
    let mut forecast1 = create_test_forecast("PROJ-001", "Project 1");
    forecast1.month1_labor_revenue_commit = 10000.0;
    forecast1.month2_labor_revenue_commit = 20000.0;
    forecast1.month3_labor_revenue_commit = 30000.0;
    forecast1.month4_labor_revenue_commit = 40000.0;
    forecast1.month5_labor_revenue_commit = 50000.0;
    forecast1.month6_labor_revenue_commit = 60000.0;
    let mut forecast2 = create_test_forecast("PROJ-002", "Project 2");
    forecast2.month1_labor_revenue_commit = 5000.0;
    forecast2.month2_labor_revenue_commit = 10000.0;
    forecast2.month3_labor_revenue_commit = 15000.0;
    forecast2.month4_labor_revenue_commit = 20000.0;
    forecast2.month5_labor_revenue_commit = 25000.0;
    forecast2.month6_labor_revenue_commit = 30000.0;

    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    ForecastRepositoryTrait::insert(&repo, forecast2);

    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_revenue_summary_by_account_executive();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].account_executive, "AE One");
    // Totals should be summed
    assert_eq!(result[0].month1_total, 15000.0); // 10000 + 5000
    assert_eq!(result[0].month2_total, 30000.0); // 20000 + 10000
    assert_eq!(result[0].month3_total, 45000.0); // 30000 + 15000
    assert_eq!(result[0].month4_total, 60000.0); // 40000 + 20000
    assert_eq!(result[0].month5_total, 75000.0); // 50000 + 25000
    assert_eq!(result[0].month6_total, 90000.0); // 60000 + 30000
}

#[test]
fn test_forecast_service_get_revenue_summary_by_account_executive_multiple_ae() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.account_executive = Some("AE One".to_string());
    let mut project2 = create_test_project("PROJ-002", "Project 2");
    project2.account_executive = Some("AE Two".to_string());
    let mut project3 = create_test_project("PROJ-003", "Project 3");
    project3.account_executive = Some("AE One".to_string());
    let mut forecast1 = create_test_forecast("PROJ-001", "Project 1");
    forecast1.month1_labor_revenue_commit = 10000.0;
    let mut forecast2 = create_test_forecast("PROJ-002", "Project 2");
    forecast2.month1_labor_revenue_commit = 20000.0;
    let mut forecast3 = create_test_forecast("PROJ-003", "Project 3");
    forecast3.month1_labor_revenue_commit = 5000.0;

    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ProjectRepositoryTrait::insert(&repo, project3);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    ForecastRepositoryTrait::insert(&repo, forecast2);
    ForecastRepositoryTrait::insert(&repo, forecast3);

    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_revenue_summary_by_account_executive();

    assert_eq!(result.len(), 2);
    // Should be sorted alphabetically
    assert_eq!(result[0].account_executive, "AE One");
    assert_eq!(result[0].month1_total, 15000.0); // 10000 + 5000
    assert_eq!(result[1].account_executive, "AE Two");
    assert_eq!(result[1].month1_total, 20000.0);
}

#[test]
fn test_forecast_service_get_revenue_summary_by_account_executive_no_ae() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.account_executive = None; // No Account Executive
    let forecast1 = create_test_forecast("PROJ-001", "Project 1");

    ProjectRepositoryTrait::insert(&repo, project1);
    ForecastRepositoryTrait::insert(&repo, forecast1);

    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_revenue_summary_by_account_executive();

    // Projects without Account Executive should be skipped
    assert_eq!(result.len(), 0);
}

#[test]
fn test_forecast_service_get_revenue_summary_by_account_executive_empty_ae() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.account_executive = Some("".to_string()); // Empty Account Executive
    let forecast1 = create_test_forecast("PROJ-001", "Project 1");

    ProjectRepositoryTrait::insert(&repo, project1);
    ForecastRepositoryTrait::insert(&repo, forecast1);

    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_revenue_summary_by_account_executive();

    // Projects with empty Account Executive should be skipped
    assert_eq!(result.len(), 0);
}

#[test]
fn test_forecast_service_get_revenue_summary_by_account_executive_no_forecast() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.account_executive = Some("AE One".to_string());
    // Don't insert a forecast

    ProjectRepositoryTrait::insert(&repo, project1);

    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_revenue_summary_by_account_executive();

    // Projects without forecasts should be skipped
    assert_eq!(result.len(), 0);
}

#[test]
fn test_forecast_service_get_revenue_summary_by_account_executive_sorting() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.account_executive = Some("Zebra, Zoe".to_string());
    let mut project2 = create_test_project("PROJ-002", "Project 2");
    project2.account_executive = Some("Apple, Adam".to_string());
    let mut project3 = create_test_project("PROJ-003", "Project 3");
    project3.account_executive = Some("Baker, Bob".to_string());
    let forecast1 = create_test_forecast("PROJ-001", "Project 1");
    let forecast2 = create_test_forecast("PROJ-002", "Project 2");
    let forecast3 = create_test_forecast("PROJ-003", "Project 3");

    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ProjectRepositoryTrait::insert(&repo, project3);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    ForecastRepositoryTrait::insert(&repo, forecast2);
    ForecastRepositoryTrait::insert(&repo, forecast3);

    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_revenue_summary_by_account_executive();

    assert_eq!(result.len(), 3);
    // Should be sorted alphabetically
    assert_eq!(result[0].account_executive, "Apple, Adam");
    assert_eq!(result[1].account_executive, "Baker, Bob");
    assert_eq!(result[2].account_executive, "Zebra, Zoe");
}

#[test]
fn test_forecast_service_get_total_revenue_forecast_empty_repository() {
    let repo = InMemoryRepository::new();
    let forecast_service = ForecastService::new(&repo);
    let (m1, m2, m3, m4, m5, m6) = forecast_service.get_total_revenue_forecast();

    assert_eq!(m1, 0.0);
    assert_eq!(m2, 0.0);
    assert_eq!(m3, 0.0);
    assert_eq!(m4, 0.0);
    assert_eq!(m5, 0.0);
    assert_eq!(m6, 0.0);
}

#[test]
fn test_forecast_service_get_total_revenue_forecast_single_forecast() {
    let repo = InMemoryRepository::new();
    let mut forecast1 = create_test_forecast("PROJ-001", "Project 1");
    forecast1.month1_labor_revenue_commit = 10000.0;
    forecast1.month2_labor_revenue_commit = 20000.0;
    forecast1.month3_labor_revenue_commit = 30000.0;
    forecast1.month4_labor_revenue_commit = 40000.0;
    forecast1.month5_labor_revenue_commit = 50000.0;
    forecast1.month6_labor_revenue_commit = 60000.0;

    ForecastRepositoryTrait::insert(&repo, forecast1);

    let forecast_service = ForecastService::new(&repo);
    let (m1, m2, m3, m4, m5, m6) = forecast_service.get_total_revenue_forecast();

    assert_eq!(m1, 10000.0);
    assert_eq!(m2, 20000.0);
    assert_eq!(m3, 30000.0);
    assert_eq!(m4, 40000.0);
    assert_eq!(m5, 50000.0);
    assert_eq!(m6, 60000.0);
}

#[test]
fn test_forecast_service_get_total_revenue_forecast_multiple_forecasts() {
    let repo = InMemoryRepository::new();
    let mut forecast1 = create_test_forecast("PROJ-001", "Project 1");
    forecast1.month1_labor_revenue_commit = 10000.0;
    forecast1.month2_labor_revenue_commit = 20000.0;
    forecast1.month3_labor_revenue_commit = 30000.0;
    forecast1.month4_labor_revenue_commit = 40000.0;
    forecast1.month5_labor_revenue_commit = 50000.0;
    forecast1.month6_labor_revenue_commit = 60000.0;

    let mut forecast2 = create_test_forecast("PROJ-002", "Project 2");
    forecast2.month1_labor_revenue_commit = 5000.0;
    forecast2.month2_labor_revenue_commit = 10000.0;
    forecast2.month3_labor_revenue_commit = 15000.0;
    forecast2.month4_labor_revenue_commit = 20000.0;
    forecast2.month5_labor_revenue_commit = 25000.0;
    forecast2.month6_labor_revenue_commit = 30000.0;

    let mut forecast3 = create_test_forecast("PROJ-003", "Project 3");
    forecast3.month1_labor_revenue_commit = 2000.0;
    forecast3.month2_labor_revenue_commit = 3000.0;
    forecast3.month3_labor_revenue_commit = 5000.0;
    forecast3.month4_labor_revenue_commit = 7000.0;
    forecast3.month5_labor_revenue_commit = 11000.0;
    forecast3.month6_labor_revenue_commit = 13000.0;

    ForecastRepositoryTrait::insert(&repo, forecast1);
    ForecastRepositoryTrait::insert(&repo, forecast2);
    ForecastRepositoryTrait::insert(&repo, forecast3);

    let forecast_service = ForecastService::new(&repo);
    let (m1, m2, m3, m4, m5, m6) = forecast_service.get_total_revenue_forecast();

    // Totals should be summed across all forecasts
    assert_eq!(m1, 17000.0); // 10000 + 5000 + 2000
    assert_eq!(m2, 33000.0); // 20000 + 10000 + 3000
    assert_eq!(m3, 50000.0); // 30000 + 15000 + 5000
    assert_eq!(m4, 67000.0); // 40000 + 20000 + 7000
    assert_eq!(m5, 86000.0); // 50000 + 25000 + 11000
    assert_eq!(m6, 103000.0); // 60000 + 30000 + 13000
}

#[test]
fn test_forecast_service_get_revenue_summary_by_sales_org_empty_repository() {
    let repo = InMemoryRepository::new();
    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_revenue_summary_by_sales_org();

    assert_eq!(result.len(), 0);
}

#[test]
fn test_forecast_service_get_revenue_summary_by_sales_org_single_org_single_project() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.sales_org = "ORG-ONE".to_string();
    let mut forecast1 = create_test_forecast("PROJ-001", "Project 1");
    forecast1.month1_labor_revenue_commit = 10000.0;
    forecast1.month2_labor_revenue_commit = 20000.0;
    forecast1.month3_labor_revenue_commit = 30000.0;
    forecast1.month4_labor_revenue_commit = 40000.0;
    forecast1.month5_labor_revenue_commit = 50000.0;
    forecast1.month6_labor_revenue_commit = 60000.0;

    ProjectRepositoryTrait::insert(&repo, project1);
    ForecastRepositoryTrait::insert(&repo, forecast1);

    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_revenue_summary_by_sales_org();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].sales_org, "ORG-ONE");
    assert_eq!(result[0].month1_total, 10000.0);
    assert_eq!(result[0].month2_total, 20000.0);
    assert_eq!(result[0].month3_total, 30000.0);
    assert_eq!(result[0].month4_total, 40000.0);
    assert_eq!(result[0].month5_total, 50000.0);
    assert_eq!(result[0].month6_total, 60000.0);
}

#[test]
fn test_forecast_service_get_revenue_summary_by_sales_org_single_org_multiple_projects() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.sales_org = "ORG-ONE".to_string();
    let mut project2 = create_test_project("PROJ-002", "Project 2");
    project2.sales_org = "ORG-ONE".to_string();
    let mut forecast1 = create_test_forecast("PROJ-001", "Project 1");
    forecast1.month1_labor_revenue_commit = 10000.0;
    forecast1.month2_labor_revenue_commit = 20000.0;
    forecast1.month3_labor_revenue_commit = 30000.0;
    forecast1.month4_labor_revenue_commit = 40000.0;
    forecast1.month5_labor_revenue_commit = 50000.0;
    forecast1.month6_labor_revenue_commit = 60000.0;
    let mut forecast2 = create_test_forecast("PROJ-002", "Project 2");
    forecast2.month1_labor_revenue_commit = 5000.0;
    forecast2.month2_labor_revenue_commit = 10000.0;
    forecast2.month3_labor_revenue_commit = 15000.0;
    forecast2.month4_labor_revenue_commit = 20000.0;
    forecast2.month5_labor_revenue_commit = 25000.0;
    forecast2.month6_labor_revenue_commit = 30000.0;

    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    ForecastRepositoryTrait::insert(&repo, forecast2);

    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_revenue_summary_by_sales_org();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].sales_org, "ORG-ONE");
    // Totals should be summed
    assert_eq!(result[0].month1_total, 15000.0); // 10000 + 5000
    assert_eq!(result[0].month2_total, 30000.0); // 20000 + 10000
    assert_eq!(result[0].month3_total, 45000.0); // 30000 + 15000
    assert_eq!(result[0].month4_total, 60000.0); // 40000 + 20000
    assert_eq!(result[0].month5_total, 75000.0); // 50000 + 25000
    assert_eq!(result[0].month6_total, 90000.0); // 60000 + 30000
}

#[test]
fn test_forecast_service_get_revenue_summary_by_sales_org_multiple_orgs() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.sales_org = "ORG-ONE".to_string();
    let mut project2 = create_test_project("PROJ-002", "Project 2");
    project2.sales_org = "ORG-TWO".to_string();
    let mut project3 = create_test_project("PROJ-003", "Project 3");
    project3.sales_org = "ORG-ONE".to_string();
    let mut forecast1 = create_test_forecast("PROJ-001", "Project 1");
    forecast1.month1_labor_revenue_commit = 10000.0;
    let mut forecast2 = create_test_forecast("PROJ-002", "Project 2");
    forecast2.month1_labor_revenue_commit = 20000.0;
    let mut forecast3 = create_test_forecast("PROJ-003", "Project 3");
    forecast3.month1_labor_revenue_commit = 5000.0;

    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ProjectRepositoryTrait::insert(&repo, project3);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    ForecastRepositoryTrait::insert(&repo, forecast2);
    ForecastRepositoryTrait::insert(&repo, forecast3);

    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_revenue_summary_by_sales_org();

    assert_eq!(result.len(), 2);
    // Should be sorted alphabetically
    assert_eq!(result[0].sales_org, "ORG-ONE");
    assert_eq!(result[0].month1_total, 15000.0); // 10000 + 5000
    assert_eq!(result[1].sales_org, "ORG-TWO");
    assert_eq!(result[1].month1_total, 20000.0);
}

#[test]
fn test_forecast_service_get_revenue_summary_by_sales_org_empty_sales_org() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.sales_org = "".to_string(); // Empty Sales Org
    let forecast1 = create_test_forecast("PROJ-001", "Project 1");

    ProjectRepositoryTrait::insert(&repo, project1);
    ForecastRepositoryTrait::insert(&repo, forecast1);

    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_revenue_summary_by_sales_org();

    // Projects with empty Sales Org should be skipped
    assert_eq!(result.len(), 0);
}

#[test]
fn test_forecast_service_get_revenue_summary_by_sales_org_no_forecast() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.sales_org = "ORG-ONE".to_string();
    // Don't insert a forecast

    ProjectRepositoryTrait::insert(&repo, project1);

    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_revenue_summary_by_sales_org();

    // Projects without forecasts should be skipped
    assert_eq!(result.len(), 0);
}

#[test]
fn test_forecast_service_get_revenue_summary_by_sales_org_sorting() {
    let repo = InMemoryRepository::new();
    let mut project1 = create_test_project("PROJ-001", "Project 1");
    project1.sales_org = "ZEBRA-ORG".to_string();
    let mut project2 = create_test_project("PROJ-002", "Project 2");
    project2.sales_org = "ALPHA-ORG".to_string();
    let mut project3 = create_test_project("PROJ-003", "Project 3");
    project3.sales_org = "BETA-ORG".to_string();
    let forecast1 = create_test_forecast("PROJ-001", "Project 1");
    let forecast2 = create_test_forecast("PROJ-002", "Project 2");
    let forecast3 = create_test_forecast("PROJ-003", "Project 3");

    ProjectRepositoryTrait::insert(&repo, project1);
    ProjectRepositoryTrait::insert(&repo, project2);
    ProjectRepositoryTrait::insert(&repo, project3);
    ForecastRepositoryTrait::insert(&repo, forecast1);
    ForecastRepositoryTrait::insert(&repo, forecast2);
    ForecastRepositoryTrait::insert(&repo, forecast3);

    let forecast_service = ForecastService::new(&repo);
    let result = forecast_service.get_revenue_summary_by_sales_org();

    assert_eq!(result.len(), 3);
    // Should be sorted alphabetically
    assert_eq!(result[0].sales_org, "ALPHA-ORG");
    assert_eq!(result[1].sales_org, "BETA-ORG");
    assert_eq!(result[2].sales_org, "ZEBRA-ORG");
}

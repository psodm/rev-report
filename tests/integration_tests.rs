mod common;

use common::*;
use rev_report::db::csv_loader::load_data_from_csvs;
use rev_report::db::repositories::in_memory_repository::InMemoryRepository;
use rev_report::services::forecast_service::ForecastService;
use rev_report::services::project_service::ProjectService;
use std::fs;

/// Integration test: Load CSV files and verify projects without forecasts
#[test]
fn test_integration_load_csv_and_find_projects_without_forecasts() {
    let repo = InMemoryRepository::new();
    let projects_path = create_temp_projects_csv();
    let forecasts_path = create_temp_forecasts_csv();
    
    // Load data from CSV files
    let result = load_data_from_csvs(
        &repo,
        projects_path.to_str().unwrap(),
        forecasts_path.to_str().unwrap(),
    );
    assert!(result.is_ok());
    let (project_count, forecast_count) = result.unwrap();
    assert_eq!(project_count, 3);
    assert_eq!(forecast_count, 3);
    
    // Create service and analyze
    let project_service = ProjectService::new(&repo);
    let projects_without_forecasts = project_service.find_projects_without_forecasts();
    
    // PROJ-001, PROJ-002, PROJ-003 all have forecasts with non-zero values
    // So none should be in the "without forecasts" list
    assert_eq!(projects_without_forecasts.len(), 0);
    
    // Cleanup
    let _ = fs::remove_file(&projects_path);
    let _ = fs::remove_file(&forecasts_path);
}

/// Integration test: Load CSV files and verify on-hold projects
#[test]
fn test_integration_load_csv_and_find_on_hold_projects() {
    let repo = InMemoryRepository::new();
    let projects_path = create_temp_projects_csv();
    let forecasts_path = create_temp_forecasts_csv();
    
    // Load data from CSV files
    let result = load_data_from_csvs(
        &repo,
        projects_path.to_str().unwrap(),
        forecasts_path.to_str().unwrap(),
    );
    assert!(result.is_ok());
    
    // Create service and analyze
    let project_service = ProjectService::new(&repo);
    let on_hold_projects = project_service.find_on_hold_projects();
    
    // PROJ-002 is on hold according to the test CSV
    assert_eq!(on_hold_projects.len(), 1);
    assert_eq!(on_hold_projects[0].project_id, "PROJ-002");
    assert_eq!(on_hold_projects[0].on_hold, true);
    assert_eq!(
        on_hold_projects[0].on_hold_comment,
        Some("On hold for review".to_string())
    );
    
    // Cleanup
    let _ = fs::remove_file(&projects_path);
    let _ = fs::remove_file(&forecasts_path);
}

/// Integration test: Load CSV files and verify revenue summary by Account Executive
#[test]
fn test_integration_load_csv_and_revenue_summary_by_account_executive() {
    let repo = InMemoryRepository::new();
    let projects_path = create_temp_projects_csv();
    let forecasts_path = create_temp_forecasts_csv();
    
    // Load data from CSV files
    let result = load_data_from_csvs(
        &repo,
        projects_path.to_str().unwrap(),
        forecasts_path.to_str().unwrap(),
    );
    assert!(result.is_ok());
    
    // Create service and analyze
    let forecast_service = ForecastService::new(&repo);
    let summaries = forecast_service.get_revenue_summary_by_account_executive();
    
    // PROJ-001 and PROJ-003 have Account Executives (AE 1 and AE 3)
    // PROJ-002 has no Account Executive
    assert_eq!(summaries.len(), 2);
    
    // Verify totals are correct
    // PROJ-001: month1=10000, month2=15000, month3=20000, month4=15000, month5=10000, month6=5000
    // PROJ-003: month1=30000, month2=35000, month3=40000, month4=35000, month5=30000, month6=15000
    // But they have different AEs, so they should be separate
    
    let ae1_summary = summaries.iter().find(|s| s.account_executive == "AE 1");
    assert!(ae1_summary.is_some());
    let ae1 = ae1_summary.unwrap();
    assert_eq!(ae1.month1_total, 10000.0);
    assert_eq!(ae1.month2_total, 15000.0);
    assert_eq!(ae1.month6_total, 5000.0);
    
    let ae3_summary = summaries.iter().find(|s| s.account_executive == "AE 3");
    assert!(ae3_summary.is_some());
    let ae3 = ae3_summary.unwrap();
    assert_eq!(ae3.month1_total, 30000.0);
    assert_eq!(ae3.month2_total, 35000.0);
    assert_eq!(ae3.month6_total, 15000.0);
    
    // Cleanup
    let _ = fs::remove_file(&projects_path);
    let _ = fs::remove_file(&forecasts_path);
}

/// Integration test: Load CSV files and verify revenue summary by Sales Org
#[test]
fn test_integration_load_csv_and_revenue_summary_by_sales_org() {
    let repo = InMemoryRepository::new();
    let projects_path = create_temp_projects_csv();
    let forecasts_path = create_temp_forecasts_csv();
    
    // Load data from CSV files
    let result = load_data_from_csvs(
        &repo,
        projects_path.to_str().unwrap(),
        forecasts_path.to_str().unwrap(),
    );
    assert!(result.is_ok());
    
    // Create service and analyze
    let forecast_service = ForecastService::new(&repo);
    let summaries = forecast_service.get_revenue_summary_by_sales_org();
    
    // PROJ-001: TEST-ORG-1, PROJ-002: TEST-ORG-2, PROJ-003: TEST-ORG-3
    assert_eq!(summaries.len(), 3);
    
    // Verify totals are correct for each Sales Org
    let org1_summary = summaries.iter().find(|s| s.sales_org == "TEST-ORG-1");
    assert!(org1_summary.is_some());
    let org1 = org1_summary.unwrap();
    // PROJ-001: month1=10000, month2=15000, month3=20000, month4=15000, month5=10000, month6=5000
    assert_eq!(org1.month1_total, 10000.0);
    assert_eq!(org1.month2_total, 15000.0);
    assert_eq!(org1.month6_total, 5000.0);
    
    let org2_summary = summaries.iter().find(|s| s.sales_org == "TEST-ORG-2");
    assert!(org2_summary.is_some());
    let org2 = org2_summary.unwrap();
    // PROJ-002: month1=20000, month2=25000, month3=30000, month4=25000, month5=20000, month6=10000
    assert_eq!(org2.month1_total, 20000.0);
    assert_eq!(org2.month2_total, 25000.0);
    assert_eq!(org2.month6_total, 10000.0);
    
    let org3_summary = summaries.iter().find(|s| s.sales_org == "TEST-ORG-3");
    assert!(org3_summary.is_some());
    let org3 = org3_summary.unwrap();
    // PROJ-003: month1=30000, month2=35000, month3=40000, month4=35000, month5=30000, month6=15000
    assert_eq!(org3.month1_total, 30000.0);
    assert_eq!(org3.month2_total, 35000.0);
    assert_eq!(org3.month6_total, 15000.0);
    
    // Cleanup
    let _ = fs::remove_file(&projects_path);
    let _ = fs::remove_file(&forecasts_path);
}

/// Integration test: Load CSV files and verify total revenue forecast
#[test]
fn test_integration_load_csv_and_total_revenue_forecast() {
    let repo = InMemoryRepository::new();
    let projects_path = create_temp_projects_csv();
    let forecasts_path = create_temp_forecasts_csv();
    
    // Load data from CSV files
    let result = load_data_from_csvs(
        &repo,
        projects_path.to_str().unwrap(),
        forecasts_path.to_str().unwrap(),
    );
    assert!(result.is_ok());
    
    // Create service and analyze
    let forecast_service = ForecastService::new(&repo);
    let (m1, m2, m3, m4, m5, m6) = forecast_service.get_total_revenue_forecast();
    
    // Total across all 3 projects:
    // PROJ-001: 10000, 15000, 20000, 15000, 10000, 5000
    // PROJ-002: 20000, 25000, 30000, 25000, 20000, 10000
    // PROJ-003: 30000, 35000, 40000, 35000, 30000, 15000
    // Totals:   60000, 75000, 90000, 75000, 60000, 30000
    assert_eq!(m1, 60000.0);
    assert_eq!(m2, 75000.0);
    assert_eq!(m3, 90000.0);
    assert_eq!(m4, 75000.0);
    assert_eq!(m5, 60000.0);
    assert_eq!(m6, 30000.0);
    
    // Cleanup
    let _ = fs::remove_file(&projects_path);
    let _ = fs::remove_file(&forecasts_path);
}

/// Integration test: Load CSV files and verify forecasts grouped by Project Manager
#[test]
fn test_integration_load_csv_and_forecasts_by_project_manager() {
    let repo = InMemoryRepository::new();
    let projects_path = create_temp_projects_csv();
    let forecasts_path = create_temp_forecasts_csv();
    
    // Load data from CSV files
    let result = load_data_from_csvs(
        &repo,
        projects_path.to_str().unwrap(),
        forecasts_path.to_str().unwrap(),
    );
    assert!(result.is_ok());
    
    // Create service and analyze
    let forecast_service = ForecastService::new(&repo);
    let grouped = forecast_service.get_forecasts_by_project_manager();
    
    // All three projects have different managers (Manager 1, Manager 2, Manager 3)
    assert_eq!(grouped.len(), 3);
    
    // Verify each manager has one forecast
    let manager1 = grouped.iter().find(|g| g.project_manager == "Manager 1");
    assert!(manager1.is_some());
    assert_eq!(manager1.unwrap().forecasts.len(), 1);
    
    let manager2 = grouped.iter().find(|g| g.project_manager == "Manager 2");
    assert!(manager2.is_some());
    assert_eq!(manager2.unwrap().forecasts.len(), 1);
    
    let manager3 = grouped.iter().find(|g| g.project_manager == "Manager 3");
    assert!(manager3.is_some());
    assert_eq!(manager3.unwrap().forecasts.len(), 1);
    
    // Cleanup
    let _ = fs::remove_file(&projects_path);
    let _ = fs::remove_file(&forecasts_path);
}

/// Integration test: Load CSV files and verify projects without Account Executive
#[test]
fn test_integration_load_csv_and_projects_without_account_executive() {
    let repo = InMemoryRepository::new();
    let projects_path = create_temp_projects_csv();
    let forecasts_path = create_temp_forecasts_csv();
    
    // Load data from CSV files
    let result = load_data_from_csvs(
        &repo,
        projects_path.to_str().unwrap(),
        forecasts_path.to_str().unwrap(),
    );
    assert!(result.is_ok());
    
    // Create service and analyze
    let project_service = ProjectService::new(&repo);
    let projects_without_ae = project_service.find_projects_without_account_executive();
    
    // PROJ-002 has no Account Executive (empty field in CSV)
    assert_eq!(projects_without_ae.len(), 1);
    assert_eq!(projects_without_ae[0].project_id, "PROJ-002");
    
    // Cleanup
    let _ = fs::remove_file(&projects_path);
    let _ = fs::remove_file(&forecasts_path);
}

/// Integration test: Load CSV files and verify all Account Executives
#[test]
fn test_integration_load_csv_and_all_account_executives() {
    let repo = InMemoryRepository::new();
    let projects_path = create_temp_projects_csv();
    let forecasts_path = create_temp_forecasts_csv();
    
    // Load data from CSV files
    let result = load_data_from_csvs(
        &repo,
        projects_path.to_str().unwrap(),
        forecasts_path.to_str().unwrap(),
    );
    assert!(result.is_ok());
    
    // Create service and analyze
    let project_service = ProjectService::new(&repo);
    let account_executives = project_service.find_all_account_executives();
    
    // PROJ-001 has "AE 1", PROJ-003 has "AE 3", PROJ-002 has no AE
    assert_eq!(account_executives.len(), 2);
    assert!(account_executives.contains(&"AE 1".to_string()));
    assert!(account_executives.contains(&"AE 3".to_string()));
    
    // Cleanup
    let _ = fs::remove_file(&projects_path);
    let _ = fs::remove_file(&forecasts_path);
}

/// Integration test: Load CSV files and verify projects grouped by Project Manager
#[test]
fn test_integration_load_csv_and_projects_by_project_manager() {
    let repo = InMemoryRepository::new();
    let projects_path = create_temp_projects_csv();
    let forecasts_path = create_temp_forecasts_csv();
    
    // Load data from CSV files
    let result = load_data_from_csvs(
        &repo,
        projects_path.to_str().unwrap(),
        forecasts_path.to_str().unwrap(),
    );
    assert!(result.is_ok());
    
    // Create service and analyze
    let project_service = ProjectService::new(&repo);
    let grouped = project_service.get_projects_by_project_manager();
    
    // All three projects have different managers
    assert_eq!(grouped.len(), 3);
    
    // Verify each manager has one project
    let manager1 = grouped.iter().find(|g| g.project_manager == "Manager 1");
    assert!(manager1.is_some());
    assert_eq!(manager1.unwrap().projects.len(), 1);
    
    // Verify the project has forecast data
    let project_data = &manager1.unwrap().projects[0];
    assert!(project_data.1.is_some()); // Should have forecast data
    let (total, remaining, currency, _class) = project_data.1.as_ref().unwrap();
    assert_eq!(*total, 100000.0);
    assert_eq!(*remaining, 75000.0);
    assert_eq!(currency, "USD");
    
    // Cleanup
    let _ = fs::remove_file(&projects_path);
    let _ = fs::remove_file(&forecasts_path);
}

/// Integration test: Load CSV with project without forecast and verify it's detected
#[test]
fn test_integration_load_csv_with_missing_forecast() {
    let repo = InMemoryRepository::new();
    let temp_dir = std::env::temp_dir();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    // Create projects CSV with 2 projects
    let projects_path = temp_dir.join(format!("test_projects_{}_{}.csv", std::process::id(), timestamp));
    let mut projects_file = std::fs::File::create(&projects_path).unwrap();
    use std::io::Write;
    writeln!(projects_file, "\"Project ID\",\"Sales Org\",\"End Customer Name\",\"Project Name\",\"Manager\",\"Other Stakeholder\",\"Start\",\"Finish\",\"On Hold Comment\",\"On Hold\"").unwrap();
    writeln!(projects_file, "\"code\",\"z_sales_org\",\"z_end_customer_name\",\"name\",\"manager\",\"z_other_stakeholder\",\"scheduleStart\",\"scheduleFinish\",\"z_ca_svcs_proj_holdcom\",\"z_ca_svcs_proj_hold\"").unwrap();
    writeln!(projects_file, "\"PROJ-001\",\"TEST-ORG-1\",\"Customer 1\",\"Project 1\",\"Manager 1\",\"AE 1\",\"2024-01-01\",\"2024-12-31\",\"\",\"false\"").unwrap();
    writeln!(projects_file, "\"PROJ-002\",\"TEST-ORG-2\",\"Customer 2\",\"Project 2\",\"Manager 2\",,\"2025-01-01\",\"2025-12-31\",\"\",\"false\"").unwrap();
    
    // Create forecasts CSV with only 1 forecast (PROJ-001)
    let forecasts_path = temp_dir.join(format!("test_forecasts_{}_{}.csv", std::process::id(), timestamp));
    let mut forecasts_file = std::fs::File::create(&forecasts_path).unwrap();
    writeln!(forecasts_file, ",,,,,,,,,,\"November 01, 2025\",\"December 01, 2025\",\"January 01, 2026\",\"February 01, 2026\",\"March 01, 2026\",\"April 01, 2026\"").unwrap();
    writeln!(forecasts_file, "Sold to Company,Project Manager,Project,ID,Class,Contract Start Date,Contract Finish Date,Contract Total Value,Contract Remaining Value,Currency,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ").unwrap();
    writeln!(forecasts_file, "Test Company 1,Manager 1,Project 1,PROJ-001,FP,01/01/2024,31/12/2024,100000.0,75000.0,USD,10000.0,15000.0,20000.0,15000.0,10000.0,5000.0").unwrap();
    
    // Load data from CSV files
    let result = load_data_from_csvs(
        &repo,
        projects_path.to_str().unwrap(),
        forecasts_path.to_str().unwrap(),
    );
    assert!(result.is_ok());
    
    // Create service and analyze
    let project_service = ProjectService::new(&repo);
    let projects_without_forecasts = project_service.find_projects_without_forecasts();
    
    // PROJ-002 should be in the list (no forecast)
    assert_eq!(projects_without_forecasts.len(), 1);
    assert_eq!(projects_without_forecasts[0].project_id, "PROJ-002");
    
    // Cleanup
    let _ = fs::remove_file(&projects_path);
    let _ = fs::remove_file(&forecasts_path);
}

/// Integration test: Load CSV with zero forecast and verify it's detected
#[test]
fn test_integration_load_csv_with_zero_forecast() {
    let repo = InMemoryRepository::new();
    let temp_dir = std::env::temp_dir();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    // Create projects CSV
    let projects_path = temp_dir.join(format!("test_projects_zero_{}_{}.csv", std::process::id(), timestamp));
    let mut projects_file = std::fs::File::create(&projects_path).unwrap();
    use std::io::Write;
    writeln!(projects_file, "\"Project ID\",\"Sales Org\",\"End Customer Name\",\"Project Name\",\"Manager\",\"Other Stakeholder\",\"Start\",\"Finish\",\"On Hold Comment\",\"On Hold\"").unwrap();
    writeln!(projects_file, "\"code\",\"z_sales_org\",\"z_end_customer_name\",\"name\",\"manager\",\"z_other_stakeholder\",\"scheduleStart\",\"scheduleFinish\",\"z_ca_svcs_proj_holdcom\",\"z_ca_svcs_proj_hold\"").unwrap();
    writeln!(projects_file, "\"PROJ-001\",\"TEST-ORG-1\",\"Customer 1\",\"Project 1\",\"Manager 1\",\"AE 1\",\"2024-01-01\",\"2024-12-31\",\"\",\"false\"").unwrap();
    
    // Create forecasts CSV with zero forecast
    let forecasts_path = temp_dir.join(format!("test_forecasts_zero_{}_{}.csv", std::process::id(), timestamp));
    let mut forecasts_file = std::fs::File::create(&forecasts_path).unwrap();
    writeln!(forecasts_file, ",,,,,,,,,,\"November 01, 2025\",\"December 01, 2025\",\"January 01, 2026\",\"February 01, 2026\",\"March 01, 2026\",\"April 01, 2026\"").unwrap();
    writeln!(forecasts_file, "Sold to Company,Project Manager,Project,ID,Class,Contract Start Date,Contract Finish Date,Contract Total Value,Contract Remaining Value,Currency,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ").unwrap();
    writeln!(forecasts_file, "Test Company 1,Manager 1,Project 1,PROJ-001,FP,01/01/2024,31/12/2024,100000.0,75000.0,USD,0.0,0.0,0.0,0.0,0.0,0.0").unwrap();
    
    // Load data from CSV files
    let result = load_data_from_csvs(
        &repo,
        projects_path.to_str().unwrap(),
        forecasts_path.to_str().unwrap(),
    );
    assert!(result.is_ok());
    
    // Create service and analyze
    let project_service = ProjectService::new(&repo);
    let projects_without_forecasts = project_service.find_projects_without_forecasts();
    
    // PROJ-001 should be in the list (zero forecast)
    assert_eq!(projects_without_forecasts.len(), 1);
    assert_eq!(projects_without_forecasts[0].project_id, "PROJ-001");
    
    // Cleanup
    let _ = fs::remove_file(&projects_path);
    let _ = fs::remove_file(&forecasts_path);
}

/// Integration test: Verify end-to-end workflow with multiple Sales Orgs and Account Executives
#[test]
fn test_integration_end_to_end_workflow() {
    let repo = InMemoryRepository::new();
    let temp_dir = std::env::temp_dir();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    // Create projects CSV with multiple Sales Orgs and Account Executives
    let projects_path = temp_dir.join(format!("test_projects_e2e_{}_{}.csv", std::process::id(), timestamp));
    let mut projects_file = std::fs::File::create(&projects_path).unwrap();
    use std::io::Write;
    writeln!(projects_file, "\"Project ID\",\"Sales Org\",\"End Customer Name\",\"Project Name\",\"Manager\",\"Other Stakeholder\",\"Start\",\"Finish\",\"On Hold Comment\",\"On Hold\"").unwrap();
    writeln!(projects_file, "\"code\",\"z_sales_org\",\"z_end_customer_name\",\"name\",\"manager\",\"z_other_stakeholder\",\"scheduleStart\",\"scheduleFinish\",\"z_ca_svcs_proj_holdcom\",\"z_ca_svcs_proj_hold\"").unwrap();
    writeln!(projects_file, "\"PROJ-001\",\"ORG-A\",\"Customer 1\",\"Project 1\",\"Manager A\",\"AE 1\",\"2024-01-01\",\"2024-12-31\",\"\",\"false\"").unwrap();
    writeln!(projects_file, "\"PROJ-002\",\"ORG-A\",\"Customer 2\",\"Project 2\",\"Manager A\",\"AE 1\",\"2025-01-01\",\"2025-12-31\",\"\",\"false\"").unwrap();
    writeln!(projects_file, "\"PROJ-003\",\"ORG-B\",\"Customer 3\",\"Project 3\",\"Manager B\",\"AE 2\",\"2026-01-01\",\"2026-12-31\",\"\",\"false\"").unwrap();
    
    // Create forecasts CSV
    let forecasts_path = temp_dir.join(format!("test_forecasts_e2e_{}_{}.csv", std::process::id(), timestamp));
    let mut forecasts_file = std::fs::File::create(&forecasts_path).unwrap();
    writeln!(forecasts_file, ",,,,,,,,,,\"November 01, 2025\",\"December 01, 2025\",\"January 01, 2026\",\"February 01, 2026\",\"March 01, 2026\",\"April 01, 2026\"").unwrap();
    writeln!(forecasts_file, "Sold to Company,Project Manager,Project,ID,Class,Contract Start Date,Contract Finish Date,Contract Total Value,Contract Remaining Value,Currency,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ,Labor Rev Committ").unwrap();
    writeln!(forecasts_file, "Company 1,Manager A,Project 1,PROJ-001,FP,01/01/2024,31/12/2024,100000.0,75000.0,USD,10000.0,15000.0,20000.0,15000.0,10000.0,5000.0").unwrap();
    writeln!(forecasts_file, "Company 2,Manager A,Project 2,PROJ-002,FP,01/01/2025,31/12/2025,200000.0,150000.0,USD,20000.0,25000.0,30000.0,25000.0,20000.0,10000.0").unwrap();
    writeln!(forecasts_file, "Company 3,Manager B,Project 3,PROJ-003,FP,01/01/2026,31/12/2026,300000.0,225000.0,USD,30000.0,35000.0,40000.0,35000.0,30000.0,15000.0").unwrap();
    
    // Load data from CSV files
    let result = load_data_from_csvs(
        &repo,
        projects_path.to_str().unwrap(),
        forecasts_path.to_str().unwrap(),
    );
    assert!(result.is_ok());
    
    // Test multiple analysis functions
    let project_service = ProjectService::new(&repo);
    let forecast_service = ForecastService::new(&repo);
    
    // 1. Verify Sales Org summaries
    let sales_org_summaries = forecast_service.get_revenue_summary_by_sales_org();
    assert_eq!(sales_org_summaries.len(), 2);
    
    let org_a = sales_org_summaries.iter().find(|s| s.sales_org == "ORG-A").unwrap();
    // PROJ-001 + PROJ-002: month1 = 10000 + 20000 = 30000
    assert_eq!(org_a.month1_total, 30000.0);
    
    let org_b = sales_org_summaries.iter().find(|s| s.sales_org == "ORG-B").unwrap();
    // PROJ-003: month1 = 30000
    assert_eq!(org_b.month1_total, 30000.0);
    
    // 2. Verify Account Executive summaries
    let ae_summaries = forecast_service.get_revenue_summary_by_account_executive();
    assert_eq!(ae_summaries.len(), 2);
    
    let ae1 = ae_summaries.iter().find(|s| s.account_executive == "AE 1").unwrap();
    // PROJ-001 + PROJ-002: month1 = 10000 + 20000 = 30000
    assert_eq!(ae1.month1_total, 30000.0);
    
    let ae2 = ae_summaries.iter().find(|s| s.account_executive == "AE 2").unwrap();
    // PROJ-003: month1 = 30000
    assert_eq!(ae2.month1_total, 30000.0);
    
    // 3. Verify total revenue forecast
    let (m1, _m2, _m3, _m4, _m5, _m6) = forecast_service.get_total_revenue_forecast();
    // Total: 10000+20000+30000 = 60000 for month1
    assert_eq!(m1, 60000.0);
    
    // 4. Verify projects by Project Manager
    let projects_by_pm = project_service.get_projects_by_project_manager();
    assert_eq!(projects_by_pm.len(), 2);
    
    let manager_a = projects_by_pm.iter().find(|g| g.project_manager == "Manager A").unwrap();
    assert_eq!(manager_a.projects.len(), 2);
    
    let manager_b = projects_by_pm.iter().find(|g| g.project_manager == "Manager B").unwrap();
    assert_eq!(manager_b.projects.len(), 1);
    
    // Cleanup
    let _ = fs::remove_file(&projects_path);
    let _ = fs::remove_file(&forecasts_path);
}


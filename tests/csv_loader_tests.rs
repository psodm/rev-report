mod common;

use common::*;
use rev_report::db::csv_loader::{
    load_data_from_csvs, load_forecasts_from_csv, load_projects_from_csv,
};
use rev_report::db::repositories::forecast_repository::ForecastRepositoryTrait;
use rev_report::db::repositories::in_memory_repository::InMemoryRepository;
use rev_report::db::repositories::project_repository::ProjectRepositoryTrait;
use std::fs;
use std::fs::File;
use std::io::Write;

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
    writeln!(file, "\"Project ID\",\"Sales Org\",\"End Customer Name\",\"Project Name\",\"Manager\",\"Other Stakeholder\",\"Start\",\"Finish\",\"On Hold Comment\",\"On Hold\"").unwrap();

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
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Labor Rev Committ")
    );

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
    let file_path = temp_dir.join(format!(
        "forecasts_empty_numeric_{}.csv",
        std::process::id()
    ));
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
    writeln!(file, "\"Project ID\",\"Sales Org\",\"End Customer Name\",\"Project Name\",\"Manager\",\"Other Stakeholder\",\"Start\",\"Finish\",\"On Hold Comment\",\"On Hold\"").unwrap();
    writeln!(file, "\"code\",\"z_sales_org\",\"z_end_customer_name\",\"name\",\"manager\",\"z_other_stakeholder\",\"scheduleStart\",\"scheduleFinish\",\"z_ca_svcs_proj_holdcom\",\"z_ca_svcs_proj_hold\"").unwrap();
    writeln!(file, "\"PROJ-001\",\"TEST-ORG-1\",\"Customer 1\",\"Project 1\",\"Manager 1\",,\"2024-01-01\",\"2024-12-31\",\"\",\"false\"").unwrap();
    writeln!(file, "\"PROJ-002\",\"TEST-ORG-2\",\"Customer 2\",\"Project 2\",\"Manager 2\",\"AE 2\",\"2025-01-01\",\"2025-12-31\",\"Comment\",\"True\"").unwrap();

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
    assert_eq!(
        project2.unwrap().account_executive,
        Some("AE 2".to_string())
    );

    // Cleanup
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_load_projects_from_csv_updates_existing() {
    let repo = InMemoryRepository::new();
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("projects_duplicate_{}.csv", std::process::id()));
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "\"Project ID\",\"Sales Org\",\"End Customer Name\",\"Project Name\",\"Manager\",\"Other Stakeholder\",\"Start\",\"Finish\",\"On Hold Comment\",\"On Hold\"").unwrap();
    writeln!(file, "\"code\",\"z_sales_org\",\"z_end_customer_name\",\"name\",\"manager\",\"z_other_stakeholder\",\"scheduleStart\",\"scheduleFinish\",\"z_ca_svcs_proj_holdcom\",\"z_ca_svcs_proj_hold\"").unwrap();
    writeln!(file, "\"PROJ-001\",\"TEST-ORG-1\",\"Original Customer\",\"Original Project\",\"Manager 1\",,\"2024-01-01\",\"2024-12-31\",\"\",\"false\"").unwrap();
    writeln!(file, "\"PROJ-001\",\"TEST-ORG-2\",\"Updated Customer\",\"Updated Project\",\"Manager 2\",\"AE 2\",\"2025-01-01\",\"2025-12-31\",\"Updated comment\",\"true\"").unwrap();

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
    writeln!(file, "\"Project ID\",\"Sales Org\",\"End Customer Name\",\"Project Name\",\"Manager\",\"Other Stakeholder\",\"Start\",\"Finish\",\"On Hold Comment\",\"On Hold\"").unwrap();
    writeln!(file, "\"code\",\"z_sales_org\",\"z_end_customer_name\",\"name\",\"manager\",\"z_other_stakeholder\",\"scheduleStart\",\"scheduleFinish\",\"z_ca_svcs_proj_holdcom\",\"z_ca_svcs_proj_hold\"").unwrap();
    writeln!(file, "\"PROJ-001\",\"TEST-ORG-1\",\"Customer 1\",\"Project 1\",\"Manager 1\",,\"2024-01-01\",\"2024-12-31\",\"\",\"false\"").unwrap();

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

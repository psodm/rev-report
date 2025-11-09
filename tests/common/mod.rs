use chrono::NaiveDate;
use rev_report::models::forecast::Forecast;
use rev_report::models::project::Project;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Helper function to create a test Project
#[allow(dead_code)]
pub fn create_test_project(project_id: &str, project_name: &str) -> Project {
    Project {
        project_id: project_id.to_string(),
        sales_org: "TEST-ORG".to_string(),
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

/// Helper function to create a test Forecast
#[allow(dead_code)]
pub fn create_test_forecast(project_id: &str, project_name: &str) -> Forecast {
    Forecast {
        sold_to_company: "Test Company".to_string(),
        project_manager: "Test Manager".to_string(),
        project_name: project_name.to_string(),
        project_id: project_id.to_string(),
        class: "Test Class".to_string(),
        start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        finish_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
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

/// Helper function to create a temporary projects CSV file
#[allow(dead_code)]
pub fn create_temp_projects_csv() -> PathBuf {
    let temp_dir = std::env::temp_dir();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let file_path = temp_dir.join(format!(
        "test_projects_{}_{}.csv",
        std::process::id(),
        timestamp
    ));

    // Remove file if it exists
    let _ = fs::remove_file(&file_path);

    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "\"Project ID\",\"Sales Org\",\"End Customer Name\",\"Project Name\",\"Manager\",\"Other Stakeholder\",\"Start\",\"Finish\",\"On Hold Comment\",\"On Hold\"").unwrap();
    writeln!(file, "\"code\",\"z_sales_org\",\"z_end_customer_name\",\"name\",\"manager\",\"z_other_stakeholder\",\"scheduleStart\",\"scheduleFinish\",\"z_ca_svcs_proj_holdcom\",\"z_ca_svcs_proj_hold\"").unwrap();
    writeln!(file, "\"PROJ-001\",\"TEST-ORG-1\",\"Test Customer 1\",\"Test Project 1\",\"Manager 1\",\"AE 1\",\"2024-01-01\",\"2024-12-31\",\"\",\"false\"").unwrap();
    writeln!(file, "\"PROJ-002\",\"TEST-ORG-2\",\"Test Customer 2\",\"Test Project 2\",\"Manager 2\",,\"2025-01-01\",\"2025-12-31\",\"On hold for review\",\"true\"").unwrap();
    writeln!(file, "\"PROJ-003\",\"TEST-ORG-3\",\"Test Customer 3\",\"Test Project 3\",\"Manager 3\",\"AE 3\",\"2026-01-01\",\"2026-12-31\",\"\",\"false\"").unwrap();

    file_path
}

/// Helper function to create a temporary forecasts CSV file
#[allow(dead_code)]
pub fn create_temp_forecasts_csv() -> PathBuf {
    let temp_dir = std::env::temp_dir();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let file_path = temp_dir.join(format!(
        "test_forecasts_{}_{}.csv",
        std::process::id(),
        timestamp
    ));

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

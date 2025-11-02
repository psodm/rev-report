use crate::db::repositories::forecast_repository::ForecastRepositoryTrait;
use crate::db::repositories::in_memory_repository::InMemoryRepository;
use crate::db::repositories::project_repository::ProjectRepositoryTrait;
use crate::models::forecast::Forecast;
use crate::models::project::Project;
use chrono::NaiveDate;
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use tracing::{debug, error, info, trace, warn};

/// Loads projects from a CSV file into the repository
pub fn load_projects_from_csv(
    repo: &InMemoryRepository,
    file_path: &str,
) -> Result<usize, Box<dyn Error>> {
    info!(file_path = %file_path, "Loading projects from CSV file");
    let file = match File::open(file_path) {
        Ok(f) => {
            debug!(file_path = %file_path, "CSV file opened successfully");
            f
        }
        Err(e) => {
            error!(file_path = %file_path, error = %e, "Failed to open CSV file");
            return Err(Box::new(e));
        }
    };

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(file));

    let mut count = 0;
    let mut skipped_rows = 0;
    for result in reader.deserialize::<Project>() {
        match result {
            Ok(project) => {
                // Skip the header row that has field names as data
                if project.project_id == "code" {
                    trace!("Skipping 'code' header row");
                    skipped_rows += 1;
                    continue;
                }
                trace!(project_id = %project.project_id, "Loading project from CSV");
                ProjectRepositoryTrait::insert(repo, project);
                count += 1;
            }
            Err(e) => {
                warn!(error = %e, "Failed to deserialize project row, skipping");
                return Err(Box::new(e));
            }
        }
    }

    info!(file_path = %file_path, count = count, skipped_rows = skipped_rows, "Successfully loaded projects from CSV");
    Ok(count)
}

/// Loads forecasts from a CSV file into the repository
pub fn load_forecasts_from_csv(
    repo: &InMemoryRepository,
    file_path: &str,
) -> Result<usize, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(false) // We'll handle headers manually
        .from_reader(BufReader::new(file));

    let mut count = 0;

    // Skip the first row (date headers)
    let mut records = reader.records();
    records.next();

    // Read the actual header row to get column positions
    let header_record = match records.next() {
        Some(Ok(record)) => record,
        Some(Err(e)) => return Err(Box::new(e)),
        None => return Err("Missing header row in CSV".into()),
    };

    // Find the position of "Labor Rev Committ" columns
    let mut labor_rev_positions = Vec::new();
    for (i, header) in header_record.iter().enumerate() {
        if header == "Labor Rev Committ" {
            labor_rev_positions.push(i);
        }
    }

    // We need at least 6 labor rev columns
    if labor_rev_positions.len() < 6 {
        return Err("CSV file must have at least 6 'Labor Rev Committ' columns".into());
    }

    let mut skipped_rows = 0;
    for result in records {
        let record = match result {
            Ok(r) => r,
            Err(e) => {
                // Skip rows that fail to parse (e.g., empty rows with wrong column count)
                warn!(error = %e, "Failed to parse CSV row, skipping");
                skipped_rows += 1;
                continue;
            }
        };

        // Parse the basic fields that have unique names
        let sold_to_company = record.get(0).unwrap_or("").to_string();
        let project_manager = record.get(1).unwrap_or("").to_string();
        let project_name = record.get(2).unwrap_or("").to_string();
        let project_id = record.get(3).unwrap_or("").to_string();

        // Skip rows with empty project_id
        if project_id.trim().is_empty() {
            trace!("Skipping row with empty project_id");
            skipped_rows += 1;
            continue;
        }

        let class = record.get(4).unwrap_or("").to_string();
        let start_date_str = record.get(5).unwrap_or("").to_string();
        let finish_date_str = record.get(6).unwrap_or("").to_string();

        trace!(project_id = %project_id, "Loading forecast from CSV");

        // Parse dates from dd/mm/yyyy format
        let start_date = parse_date_from_dd_mm_yyyy(&start_date_str).unwrap_or_else(|| {
            // Default to epoch if parsing fails
            warn!(project_id = %project_id, start_date_str = %start_date_str, "Failed to parse start date, using default");
            NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
        });
        let finish_date = parse_date_from_dd_mm_yyyy(&finish_date_str).unwrap_or_else(|| {
            // Default to epoch if parsing fails
            warn!(project_id = %project_id, finish_date_str = %finish_date_str, "Failed to parse finish date, using default");
            NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
        });

        // Parse numeric fields
        let contract_total_value = record.get(7).unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        let contract_remaining_value = record.get(8).unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        let currency = record.get(9).unwrap_or("").to_string();

        // Parse labor revenue commit fields by position
        let month1_labor_revenue_commit = record
            .get(labor_rev_positions[0])
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap_or(0.0);
        let month2_labor_revenue_commit = record
            .get(labor_rev_positions[1])
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap_or(0.0);
        let month3_labor_revenue_commit = record
            .get(labor_rev_positions[2])
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap_or(0.0);
        let month4_labor_revenue_commit = record
            .get(labor_rev_positions[3])
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap_or(0.0);
        let month5_labor_revenue_commit = record
            .get(labor_rev_positions[4])
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap_or(0.0);
        let month6_labor_revenue_commit = record
            .get(labor_rev_positions[5])
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap_or(0.0);

        // Skip rows with ID header - already checked above for empty project_id
        if project_id == "ID" {
            trace!("Skipping row with 'ID' header");
            skipped_rows += 1;
            continue;
        }

        let forecast = Forecast {
            sold_to_company,
            project_manager,
            project_name,
            project_id,
            class,
            start_date,
            finish_date,
            contract_total_value,
            contract_remaining_value,
            currency,
            month1_labor_revenue_commit,
            month2_labor_revenue_commit,
            month3_labor_revenue_commit,
            month4_labor_revenue_commit,
            month5_labor_revenue_commit,
            month6_labor_revenue_commit,
        };

        ForecastRepositoryTrait::insert(repo, forecast);
        count += 1;
    }

    info!(file_path = %file_path, count = count, skipped_rows = skipped_rows, "Successfully loaded forecasts from CSV");
    Ok(count)
}

/// Parses a date string in dd/mm/yyyy format to NaiveDate
fn parse_date_from_dd_mm_yyyy(date_str: &str) -> Option<NaiveDate> {
    if date_str.is_empty() {
        return None;
    }

    let parts: Vec<&str> = date_str.split('/').collect();
    if parts.len() != 3 {
        return None;
    }

    let day = parts[0].parse::<u32>().ok()?;
    let month = parts[1].parse::<u32>().ok()?;
    let year = parts[2].parse::<i32>().ok()?;

    NaiveDate::from_ymd_opt(year, month, day)
}

/// Loads both projects and forecasts from CSV files into the repository
pub fn load_data_from_csvs(
    repo: &InMemoryRepository,
    projects_path: &str,
    forecasts_path: &str,
) -> Result<(usize, usize), Box<dyn Error>> {
    info!(projects_path = %projects_path, forecasts_path = %forecasts_path, "Loading data from CSV files");
    let project_count = match load_projects_from_csv(repo, projects_path) {
        Ok(count) => {
            debug!(count = count, "Projects loaded successfully");
            count
        }
        Err(e) => {
            error!(projects_path = %projects_path, error = %e, "Failed to load projects from CSV");
            return Err(e);
        }
    };

    let forecast_count = match load_forecasts_from_csv(repo, forecasts_path) {
        Ok(count) => {
            debug!(count = count, "Forecasts loaded successfully");
            count
        }
        Err(e) => {
            error!(forecasts_path = %forecasts_path, error = %e, "Failed to load forecasts from CSV");
            return Err(e);
        }
    };

    info!(
        project_count = project_count,
        forecast_count = forecast_count,
        "Successfully loaded all data from CSV files"
    );

    Ok((project_count, forecast_count))
}

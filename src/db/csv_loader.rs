use crate::db::repositories::forecast_repository::ForecastRepositoryTrait;
use crate::db::repositories::in_memory_repository::InMemoryRepository;
use crate::db::repositories::project_repository::ProjectRepositoryTrait;
use crate::models::forecast::Forecast;
use crate::models::project::Project;
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

/// Loads projects from a CSV file into the repository
pub fn load_projects_from_csv(
    repo: &InMemoryRepository,
    file_path: &str,
) -> Result<usize, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(file));

    let mut count = 0;
    for result in reader.deserialize() {
        let project: Project = result?;
        // Skip the header row that has field names as data
        if project.project_id == "code" {
            continue;
        }
        ProjectRepositoryTrait::insert(repo, project);
        count += 1;
    }

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

    for result in records {
        let record = match result {
            Ok(r) => r,
            Err(_e) => {
                // Skip rows that fail to parse (e.g., empty rows with wrong column count)
                continue;
            }
        };

        // Parse the basic fields that have unique names
        let sold_to_company = record.get(0).unwrap_or("").to_string();
        let project_manager = record.get(1).unwrap_or("").to_string();
        let project_name = record.get(2).unwrap_or("").to_string();
        let project_id = record.get(3).unwrap_or("").to_string();
        let class = record.get(4).unwrap_or("").to_string();
        let start_date = record.get(5).unwrap_or("").to_string();
        let finish_date = record.get(6).unwrap_or("").to_string();

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

        // Skip rows with empty project_id (header rows or empty data)
        if project_id.is_empty() || project_id == "ID" {
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

    Ok(count)
}

/// Loads both projects and forecasts from CSV files into the repository
pub fn load_data_from_csvs(
    repo: &InMemoryRepository,
    projects_path: &str,
    forecasts_path: &str,
) -> Result<(usize, usize), Box<dyn Error>> {
    let project_count = load_projects_from_csv(repo, projects_path)?;
    let forecast_count = load_forecasts_from_csv(repo, forecasts_path)?;
    Ok((project_count, forecast_count))
}

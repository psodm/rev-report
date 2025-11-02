use crate::db::repositories::forecast_repository::ForecastRepositoryTrait;
use crate::db::repositories::in_memory_repository::InMemoryRepository;
use crate::db::repositories::project_repository::ProjectRepositoryTrait;
use crate::models::forecast::Forecast;
use crate::services::project_service::ProjectService;
use std::collections::HashMap;
use tracing::{debug, info, trace};

pub struct ForecastService<'a> {
    repository: &'a InMemoryRepository,
}

#[derive(Clone, Debug)]
pub struct ForecastByProjectManager {
    pub project_manager: String,
    pub forecasts: Vec<(Forecast, Option<String>)>, // Forecast and customer name
}

#[derive(Clone, Debug)]
pub struct RevenueSummaryByAccountExecutive {
    pub account_executive: String,
    pub month1_total: f64,
    pub month2_total: f64,
    pub month3_total: f64,
    pub month4_total: f64,
    pub month5_total: f64,
    pub month6_total: f64,
}

impl<'a> ForecastService<'a> {
    pub fn new(repository: &'a InMemoryRepository) -> Self {
        Self { repository }
    }

    /// Groups forecasts by project manager
    /// Returns a vector of ForecastByProjectManager, sorted by project manager name
    pub fn get_forecasts_by_project_manager(&self) -> Vec<ForecastByProjectManager> {
        info!("Grouping forecasts by Project Manager");
        let all_forecasts = ForecastRepositoryTrait::find_all(self.repository);
        let mut grouped: HashMap<String, Vec<(Forecast, Option<String>)>> = HashMap::new();

        trace!(total_forecasts = all_forecasts.len(), "Grouping forecasts");
        for forecast in all_forecasts {
            // Get customer name from project
            let customer_name =
                ProjectRepositoryTrait::find_by_id(self.repository, &forecast.project_id)
                    .map(|project| project.end_customer_name.clone());

            // Extract project manager name (remove account id part)
            let project_manager_name =
                ProjectService::extract_project_manager_name(&forecast.project_manager);

            grouped
                .entry(project_manager_name.to_string())
                .or_insert_with(Vec::new)
                .push((forecast, customer_name));
        }

        // Convert to vector and sort by project manager name
        let mut result: Vec<ForecastByProjectManager> = grouped
            .into_iter()
            .map(|(project_manager, forecasts)| ForecastByProjectManager {
                project_manager,
                forecasts,
            })
            .collect();

        result.sort_by(|a, b| a.project_manager.cmp(&b.project_manager));

        debug!(
            manager_count = result.len(),
            "Grouped forecasts by Project Manager"
        );
        info!(
            manager_count = result.len(),
            "Retrieved forecasts grouped by Project Manager"
        );
        result
    }

    /// Calculates revenue summaries grouped by Account Executive
    /// Returns a vector of RevenueSummaryByAccountExecutive, sorted by Account Executive name
    pub fn get_revenue_summary_by_account_executive(&self) -> Vec<RevenueSummaryByAccountExecutive> {
        let span = tracing::info_span!("get_revenue_summary_by_account_executive");
        let _guard = span.enter();
        info!("Calculating revenue summaries by Account Executive");

        let all_projects = ProjectRepositoryTrait::find_all(self.repository);
        let mut summaries: HashMap<String, (f64, f64, f64, f64, f64, f64)> = HashMap::new();

        trace!(total_projects = all_projects.len(), "Processing projects for revenue summary");

        for project in all_projects {
            // Get the account executive name, skip if empty or None
            let account_executive = match &project.account_executive {
                Some(ae) if !ae.trim().is_empty() => ae.trim().to_string(),
                _ => continue, // Skip projects without an Account Executive
            };

            // Get the forecast for this project
            let forecast = match ForecastRepositoryTrait::find_by_id(self.repository, &project.project_id) {
                Some(f) => f,
                None => continue, // Skip projects without a forecast
            };

            // Get or create the summary for this Account Executive
            let summary = summaries.entry(account_executive.clone()).or_insert((0.0, 0.0, 0.0, 0.0, 0.0, 0.0));

            // Add this project's forecast amounts to the totals
            summary.0 += forecast.month1_labor_revenue_commit;
            summary.1 += forecast.month2_labor_revenue_commit;
            summary.2 += forecast.month3_labor_revenue_commit;
            summary.3 += forecast.month4_labor_revenue_commit;
            summary.4 += forecast.month5_labor_revenue_commit;
            summary.5 += forecast.month6_labor_revenue_commit;

            trace!(
                account_executive = %account_executive,
                project_id = %project.project_id,
                "Added forecast to Account Executive summary"
            );
        }

        // Convert to vector and sort by Account Executive name
        let mut result: Vec<RevenueSummaryByAccountExecutive> = summaries
            .into_iter()
            .map(|(account_executive, (m1, m2, m3, m4, m5, m6))| {
                RevenueSummaryByAccountExecutive {
                    account_executive,
                    month1_total: m1,
                    month2_total: m2,
                    month3_total: m3,
                    month4_total: m4,
                    month5_total: m5,
                    month6_total: m6,
                }
            })
            .collect();

        result.sort_by(|a, b| a.account_executive.cmp(&b.account_executive));

        debug!(ae_count = result.len(), "Calculated revenue summaries by Account Executive");
        info!(ae_count = result.len(), "Retrieved revenue summaries grouped by Account Executive");

        result
    }
}

use crate::db::repositories::forecast_repository::ForecastRepositoryTrait;
use crate::db::repositories::in_memory_repository::InMemoryRepository;
use crate::db::repositories::project_repository::ProjectRepositoryTrait;
use crate::models::forecast::Forecast;
use crate::services::project_service::ProjectService;
use std::collections::HashMap;

pub struct ForecastService<'a> {
    repository: &'a InMemoryRepository,
}

#[derive(Clone, Debug)]
pub struct ForecastByProjectManager {
    pub project_manager: String,
    pub forecasts: Vec<(Forecast, Option<String>)>, // Forecast and customer name
}

impl<'a> ForecastService<'a> {
    pub fn new(repository: &'a InMemoryRepository) -> Self {
        Self { repository }
    }

    /// Groups forecasts by project manager
    /// Returns a vector of ForecastByProjectManager, sorted by project manager name
    pub fn get_forecasts_by_project_manager(&self) -> Vec<ForecastByProjectManager> {
        let all_forecasts = ForecastRepositoryTrait::find_all(self.repository);
        let mut grouped: HashMap<String, Vec<(Forecast, Option<String>)>> = HashMap::new();

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

        result
    }
}

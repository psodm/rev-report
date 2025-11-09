use crate::db::repositories::forecast_repository::ForecastRepositoryTrait;
use crate::db::repositories::in_memory_repository::InMemoryRepository;
use crate::db::repositories::project_repository::ProjectRepositoryTrait;
use crate::models::project::Project;
use std::collections::HashMap;
use tracing::{debug, info, trace};

#[derive(Clone, Debug)]
pub struct ProjectByProjectManager {
    pub project_manager: String,
    pub projects: Vec<(Project, Option<(f64, f64, String, String)>)>, // Project and (contract_total_value, contract_remaining_value, currency, class)
}

pub struct ProjectService<'a> {
    pub(crate) repository: &'a InMemoryRepository,
}

impl<'a> ProjectService<'a> {
    pub fn new(repository: &'a InMemoryRepository) -> Self {
        Self { repository }
    }

    /// Finds all projects that do not have a corresponding forecast
    /// This includes:
    /// 1. Projects with no forecast entry in ForecastRepository
    /// 2. Projects with a forecast entry but all month values sum to 0
    pub fn find_projects_without_forecasts(&self) -> Vec<Project> {
        info!("Finding projects without forecasts");
        let all_projects = ProjectRepositoryTrait::find_all(self.repository);
        let mut projects_without_forecasts = Vec::new();

        trace!(
            total_projects = all_projects.len(),
            "Checking projects for forecasts"
        );
        for project in all_projects {
            // Check if this project has a corresponding forecast
            match ForecastRepositoryTrait::find_by_id(self.repository, &project.project_id) {
                None => {
                    // No forecast found, add to the list
                    trace!(project_id = %project.project_id, "Project has no forecast entry");
                    projects_without_forecasts.push(project);
                }
                Some(forecast) => {
                    // Forecast exists, check if all month values sum to 0
                    let total_forecast = forecast.month1_labor_revenue_commit
                        + forecast.month2_labor_revenue_commit
                        + forecast.month3_labor_revenue_commit
                        + forecast.month4_labor_revenue_commit
                        + forecast.month5_labor_revenue_commit
                        + forecast.month6_labor_revenue_commit;

                    if total_forecast == 0.0 {
                        // All month forecasts are 0, add to the list
                        trace!(project_id = %project.project_id, "Project has zero forecast");
                        projects_without_forecasts.push(project);
                    }
                    // Otherwise, forecast exists with non-zero values, skip this project
                }
            }
        }

        info!(
            count = projects_without_forecasts.len(),
            "Found projects without forecasts"
        );
        projects_without_forecasts
    }

    /// Finds all projects that are on hold
    pub fn find_on_hold_projects(&self) -> Vec<Project> {
        info!("Finding on-hold projects");
        let all_projects = ProjectRepositoryTrait::find_all(self.repository);
        let on_hold_projects: Vec<Project> = all_projects
            .into_iter()
            .filter(|project| {
                if project.on_hold {
                    trace!(project_id = %project.project_id, "Project is on hold");
                }
                project.on_hold
            })
            .collect();
        info!(count = on_hold_projects.len(), "Found on-hold projects");
        on_hold_projects
    }

    /// Finds all projects that do not have an Account Executive (Other Stakeholder field is empty)
    pub fn find_projects_without_account_executive(&self) -> Vec<Project> {
        info!("Finding projects without Account Executive");
        let all_projects = ProjectRepositoryTrait::find_all(self.repository);
        let projects_without_ae: Vec<Project> = all_projects
            .into_iter()
            .filter(|project| {
                let missing = project.account_executive.is_none()
                    || project
                        .account_executive
                        .as_ref()
                        .map(|s| s.trim().is_empty())
                        .unwrap_or(true);
                if missing {
                    trace!(project_id = %project.project_id, "Project has no Account Executive");
                }
                missing
            })
            .collect();
        info!(
            count = projects_without_ae.len(),
            "Found projects without Account Executive"
        );
        projects_without_ae
    }

    /// Finds all unique Account Executives from the Other Stakeholder field
    /// Returns a sorted vector of unique Account Executive names
    pub fn find_all_account_executives(&self) -> Vec<String> {
        use std::collections::HashSet;
        info!("Finding all Account Executives");
        let all_projects = ProjectRepositoryTrait::find_all(self.repository);
        let mut account_executives = HashSet::new();

        for project in all_projects {
            if let Some(ae) = &project.account_executive {
                let trimmed_ae = ae.trim();
                if !trimmed_ae.is_empty() {
                    account_executives.insert(trimmed_ae.to_string());
                }
            }
        }

        let mut result: Vec<String> = account_executives.into_iter().collect();
        result.sort();
        debug!(count = result.len(), "Found unique Account Executives");
        info!(count = result.len(), "Found all Account Executives");
        result
    }

    /// Extracts just the name from a project manager field
    /// Format: "Last, First{account_id}" -> returns "Last, First"
    pub fn extract_project_manager_name(project_manager: &str) -> &str {
        // Find the index of '{' if it exists
        if let Some(index) = project_manager.find('{') {
            // Return everything before the '{'
            &project_manager[..index]
        } else {
            // No '{' found, return the whole string
            project_manager
        }
    }

    /// Sanitizes on-hold comments by replacing newline characters with spaces
    /// This prevents formatting issues when displaying comments in the terminal
    pub fn sanitize_on_hold_comment(comment: &str) -> String {
        comment
            .replace('\n', " ")
            .replace('\r', " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string()
    }

    /// Groups projects by project manager
    /// Returns a vector of ProjectByProjectManager, sorted by project manager name
    /// For each project, includes optional contract values, currency, and class from the forecast if available
    pub fn get_projects_by_project_manager(&self) -> Vec<ProjectByProjectManager> {
        let span = tracing::info_span!("get_projects_by_project_manager");
        let _guard = span.enter();
        info!("Grouping projects by Project Manager");
        let all_projects = ProjectRepositoryTrait::find_all(self.repository);
        let mut grouped: HashMap<String, Vec<(Project, Option<(f64, f64, String, String)>)>> =
            HashMap::new();

        trace!(total_projects = all_projects.len(), "Grouping projects");
        for project in all_projects {
            // Extract project manager name (remove account id part)
            let project_manager_name = Self::extract_project_manager_name(&project.project_manager);

            // Get contract values, currency, and class from forecast if available
            let contract_values =
                ForecastRepositoryTrait::find_by_id(self.repository, &project.project_id).map(
                    |forecast| {
                        (
                            forecast.contract_total_value,
                            forecast.contract_remaining_value,
                            forecast.currency.clone(),
                            forecast.class.clone(),
                        )
                    },
                );

            grouped
                .entry(project_manager_name.to_string())
                .or_insert_with(Vec::new)
                .push((project, contract_values));
        }

        // Convert to vector and sort by project manager name
        let mut result: Vec<ProjectByProjectManager> = grouped
            .into_iter()
            .map(|(project_manager, projects)| ProjectByProjectManager {
                project_manager,
                projects,
            })
            .collect();

        result.sort_by(|a, b| a.project_manager.cmp(&b.project_manager));

        debug!(
            manager_count = result.len(),
            "Grouped projects by Project Manager"
        );
        info!(
            manager_count = result.len(),
            "Retrieved projects grouped by Project Manager"
        );
        result
    }

    /// Groups on-hold projects by Account Executive
    /// Returns a HashMap where the key is the Account Executive name (or "No Account Executive" for projects without one)
    /// and the value is a vector of on-hold projects for that Account Executive
    pub fn get_on_hold_projects_by_account_executive(&self) -> HashMap<String, Vec<Project>> {
        let span = tracing::info_span!("get_on_hold_projects_by_account_executive");
        let _guard = span.enter();
        info!("Grouping on-hold projects by Account Executive");

        let on_hold_projects = self.find_on_hold_projects();
        let mut grouped: HashMap<String, Vec<Project>> = HashMap::new();

        trace!(
            total_on_hold = on_hold_projects.len(),
            "Grouping on-hold projects by Account Executive"
        );

        for project in on_hold_projects {
            let account_executive = match &project.account_executive {
                Some(ae) if !ae.trim().is_empty() => ae.trim().to_string(),
                _ => "No Account Executive".to_string(),
            };

            grouped
                .entry(account_executive)
                .or_insert_with(Vec::new)
                .push(project);
        }

        debug!(
            ae_count = grouped.len(),
            "Grouped on-hold projects by Account Executive"
        );
        info!(
            ae_count = grouped.len(),
            "Retrieved on-hold projects grouped by Account Executive"
        );

        grouped
    }
}

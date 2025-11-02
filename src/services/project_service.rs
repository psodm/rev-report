use crate::db::repositories::forecast_repository::ForecastRepositoryTrait;
use crate::db::repositories::in_memory_repository::InMemoryRepository;
use crate::db::repositories::project_repository::ProjectRepositoryTrait;
use crate::models::project::Project;

pub struct ProjectService<'a> {
    repository: &'a InMemoryRepository,
}

impl<'a> ProjectService<'a> {
    pub fn new(repository: &'a InMemoryRepository) -> Self {
        Self { repository }
    }

    /// Finds all projects that do not have a corresponding forecast
    pub fn find_projects_without_forecasts(&self) -> Vec<Project> {
        let all_projects = ProjectRepositoryTrait::find_all(self.repository);
        let mut projects_without_forecasts = Vec::new();

        for project in all_projects {
            // Check if this project has a corresponding forecast
            match ForecastRepositoryTrait::find_by_id(self.repository, &project.project_id) {
                None => {
                    // No forecast found, add to the list
                    projects_without_forecasts.push(project);
                }
                Some(_) => {
                    // Forecast exists, skip this project
                }
            }
        }

        projects_without_forecasts
    }

    /// Finds all projects that are on hold
    pub fn find_on_hold_projects(&self) -> Vec<Project> {
        let all_projects = ProjectRepositoryTrait::find_all(self.repository);
        all_projects
            .into_iter()
            .filter(|project| project.on_hold)
            .collect()
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
}

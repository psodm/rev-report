use crate::db::repositories::forecast_repository::{ForecastRepository, ForecastRepositoryTrait};
use crate::db::repositories::project_repository::{ProjectRepository, ProjectRepositoryTrait};
use tracing::info;

pub struct InMemoryRepository {
    project_repository: ProjectRepository,
    forecast_repository: ForecastRepository,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        info!("Creating new InMemoryRepository");
        Self {
            project_repository: ProjectRepository::new(),
            forecast_repository: ForecastRepository::new(),
        }
    }
}

impl ProjectRepositoryTrait for InMemoryRepository {
    fn insert(&self, project: crate::models::project::Project) {
        self.project_repository.insert(project);
    }

    fn find_all(&self) -> Vec<crate::models::project::Project> {
        self.project_repository.find_all()
    }

    fn find_by_id(&self, project_id: &str) -> Option<crate::models::project::Project> {
        self.project_repository.find_by_id(project_id)
    }

    fn delete_by_id(&self, project_id: &str) -> bool {
        self.project_repository.delete_by_id(project_id)
    }
}

impl ForecastRepositoryTrait for InMemoryRepository {
    fn insert(&self, forecast: crate::models::forecast::Forecast) {
        self.forecast_repository.insert(forecast);
    }

    fn find_all(&self) -> Vec<crate::models::forecast::Forecast> {
        self.forecast_repository.find_all()
    }

    fn find_by_id(&self, project_id: &str) -> Option<crate::models::forecast::Forecast> {
        self.forecast_repository.find_by_id(project_id)
    }
}

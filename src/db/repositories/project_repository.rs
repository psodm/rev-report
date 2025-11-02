use crate::models::project::Project;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{debug, info, trace, warn};

pub trait ProjectRepositoryTrait {
    fn insert(&self, project: Project);
    fn find_all(&self) -> Vec<Project>;
    fn find_by_id(&self, project_id: &str) -> Option<Project>;
    fn delete_by_id(&self, project_id: &str) -> bool;
}

pub struct ProjectRepository {
    data: Arc<Mutex<HashMap<String, Project>>>,
}

impl ProjectRepository {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl ProjectRepositoryTrait for ProjectRepository {
    fn insert(&self, project: Project) {
        let project_id = project.project_id.clone();
        trace!(project_id = %project_id, "Inserting project into repository");
        match self.data.lock() {
            Ok(mut data) => {
                let was_update = data.contains_key(&project_id);
                data.insert(project_id.clone(), project);
                if was_update {
                    debug!(project_id = %project_id, "Updated existing project");
                } else {
                    info!(project_id = %project_id, "Inserted new project");
                }
            }
            Err(poison_error) => {
                // Mutex was poisoned - another thread panicked while holding the lock
                // Recover the lock by getting the poisoned guard
                warn!(project_id = %project_id, "Mutex poisoned, recovering lock");
                let mut data = poison_error.into_inner();
                data.insert(project_id.clone(), project);
                info!(project_id = %project_id, "Project inserted after mutex recovery");
            }
        }
    }

    fn find_all(&self) -> Vec<Project> {
        trace!("Finding all projects");
        let projects: Vec<Project> = match self.data.lock() {
            Ok(data) => data.values().cloned().collect(),
            Err(poison_error) => {
                // Recover from poison - data is still valid
                warn!("Mutex poisoned during find_all, recovering lock");
                let data = poison_error.into_inner();
                data.values().cloned().collect()
            }
        };
        debug!(count = projects.len(), "Retrieved all projects");
        projects
    }

    fn find_by_id(&self, project_id: &str) -> Option<Project> {
        trace!(project_id = %project_id, "Finding project by ID");
        let result = match self.data.lock() {
            Ok(data) => data.get(project_id).cloned(),
            Err(poison_error) => {
                // Recover from poison - data is still valid
                warn!(project_id = %project_id, "Mutex poisoned during find_by_id, recovering lock");
                let data = poison_error.into_inner();
                data.get(project_id).cloned()
            }
        };
        if result.is_some() {
            debug!(project_id = %project_id, "Project found");
        } else {
            debug!(project_id = %project_id, "Project not found");
        }
        result
    }

    fn delete_by_id(&self, project_id: &str) -> bool {
        info!(project_id = %project_id, "Deleting project by ID");
        let deleted = match self.data.lock() {
            Ok(mut data) => {
                data.remove(project_id).is_some()
            }
            Err(poison_error) => {
                // Recover from poison - data is still valid
                warn!(project_id = %project_id, "Mutex poisoned during delete_by_id, recovering lock");
                let mut data = poison_error.into_inner();
                data.remove(project_id).is_some()
            }
        };
        if deleted {
            info!(project_id = %project_id, "Project deleted successfully");
        } else {
            debug!(project_id = %project_id, "Project not found for deletion");
        }
        deleted
    }
}

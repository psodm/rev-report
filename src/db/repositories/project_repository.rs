use crate::models::project::Project;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
        match self.data.lock() {
            Ok(mut data) => {
                data.insert(project_id, project);
            }
            Err(poison_error) => {
                // Mutex was poisoned - another thread panicked while holding the lock
                // Recover the lock by getting the poisoned guard
                let mut data = poison_error.into_inner();
                data.insert(project_id, project);
            }
        }
    }

    fn find_all(&self) -> Vec<Project> {
        match self.data.lock() {
            Ok(data) => data.values().cloned().collect(),
            Err(poison_error) => {
                // Recover from poison - data is still valid
                let data = poison_error.into_inner();
                data.values().cloned().collect()
            }
        }
    }

    fn find_by_id(&self, project_id: &str) -> Option<Project> {
        match self.data.lock() {
            Ok(data) => data.get(project_id).cloned(),
            Err(poison_error) => {
                // Recover from poison - data is still valid
                let data = poison_error.into_inner();
                data.get(project_id).cloned()
            }
        }
    }

    fn delete_by_id(&self, project_id: &str) -> bool {
        match self.data.lock() {
            Ok(mut data) => {
                data.remove(project_id).is_some()
            }
            Err(poison_error) => {
                // Recover from poison - data is still valid
                let mut data = poison_error.into_inner();
                data.remove(project_id).is_some()
            }
        }
    }
}

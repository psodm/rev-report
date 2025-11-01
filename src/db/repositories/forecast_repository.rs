use crate::models::forecast::Forecast;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub trait ForecastRepositoryTrait {
    fn insert(&self, forecast: Forecast);
    fn find_all(&self) -> Vec<Forecast>;
    fn find_by_id(&self, project_id: &str) -> Option<Forecast>;
}

pub struct ForecastRepository {
    data: Arc<Mutex<HashMap<String, Forecast>>>,
}

impl ForecastRepository {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl ForecastRepositoryTrait for ForecastRepository {
    fn insert(&self, forecast: Forecast) {
        let project_id = forecast.project_id.clone();
        match self.data.lock() {
            Ok(mut data) => {
                data.insert(project_id, forecast);
            }
            Err(poison_error) => {
                // Mutex was poisoned - another thread panicked while holding the lock
                // Recover the lock by getting the poisoned guard
                let mut data = poison_error.into_inner();
                data.insert(project_id, forecast);
            }
        }
    }

    fn find_all(&self) -> Vec<Forecast> {
        match self.data.lock() {
            Ok(data) => data.values().cloned().collect(),
            Err(poison_error) => {
                // Recover from poison - data is still valid
                let data = poison_error.into_inner();
                data.values().cloned().collect()
            }
        }
    }

    fn find_by_id(&self, project_id: &str) -> Option<Forecast> {
        match self.data.lock() {
            Ok(data) => data.get(project_id).cloned(),
            Err(poison_error) => {
                // Recover from poison - data is still valid
                let data = poison_error.into_inner();
                data.get(project_id).cloned()
            }
        }
    }
}

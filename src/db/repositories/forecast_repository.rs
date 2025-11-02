use crate::models::forecast::Forecast;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{debug, info, trace, warn};

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
        trace!(project_id = %project_id, "Inserting forecast into repository");
        match self.data.lock() {
            Ok(mut data) => {
                let was_update = data.contains_key(&project_id);
                data.insert(project_id.clone(), forecast);
                if was_update {
                    debug!(project_id = %project_id, "Updated existing forecast");
                } else {
                    info!(project_id = %project_id, "Inserted new forecast");
                }
            }
            Err(poison_error) => {
                // Mutex was poisoned - another thread panicked while holding the lock
                // Recover the lock by getting the poisoned guard
                warn!(project_id = %project_id, "Mutex poisoned, recovering lock");
                let mut data = poison_error.into_inner();
                data.insert(project_id.clone(), forecast);
                info!(project_id = %project_id, "Forecast inserted after mutex recovery");
            }
        }
    }

    fn find_all(&self) -> Vec<Forecast> {
        trace!("Finding all forecasts");
        let forecasts: Vec<Forecast> = match self.data.lock() {
            Ok(data) => data.values().cloned().collect(),
            Err(poison_error) => {
                // Recover from poison - data is still valid
                warn!("Mutex poisoned during find_all, recovering lock");
                let data = poison_error.into_inner();
                data.values().cloned().collect()
            }
        };
        debug!(count = forecasts.len(), "Retrieved all forecasts");
        forecasts
    }

    fn find_by_id(&self, project_id: &str) -> Option<Forecast> {
        trace!(project_id = %project_id, "Finding forecast by ID");
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
            debug!(project_id = %project_id, "Forecast found");
        } else {
            debug!(project_id = %project_id, "Forecast not found");
        }
        result
    }
}

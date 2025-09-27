use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::StateBackend;

#[derive(Debug, Serialize, Deserialize)]
pub struct InMemoryBackend {
    last_ts: Mutex<u128>,
    counters: Mutex<HashMap<u128, u128>>,
}

impl InMemoryBackend {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            last_ts: Mutex::new(0),
            counters: Mutex::new(HashMap::new()),
        })
    }
}

impl StateBackend for InMemoryBackend {
    fn get_last_timestamp(&self) -> u128 {
        *self.last_ts.lock().unwrap()
    }

    fn set_last_timestamp(&self, ts: u128) {
        *self.last_ts.lock().unwrap() = ts;
    }

    fn get_counter(&self, context: u128) -> u128 {
        *self.counters.lock().unwrap().get(&context).unwrap_or(&0)
    }

    fn set_counter(&self, context: u128, counter: u128) {
        self.counters.lock().unwrap().insert(context, counter);
    }

    fn clear_counters(&self) {
        self.counters.lock().unwrap().clear();
    }
}

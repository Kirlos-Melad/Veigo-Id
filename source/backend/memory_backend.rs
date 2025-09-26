use std::{collections::HashMap, sync::Mutex};

use super::StateBackend;

#[derive(Debug)]
pub struct MemoryState {
    last_ts: Mutex<u128>,
    counters: Mutex<HashMap<u128, u128>>,
}

impl MemoryState {
    pub fn new() -> Self {
        Self {
            last_ts: Mutex::new(0),
            counters: Mutex::new(HashMap::new()),
        }
    }
}

impl StateBackend for MemoryState {
    fn get_last_ts(&self) -> u128 {
        *self.last_ts.lock().unwrap()
    }
    fn set_last_ts(&self, ts: u128) {
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

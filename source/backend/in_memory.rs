use crate::errors::VeigoIdError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct InMemoryBackend {
    // We wrap the entire state in ONE Mutex to guarantee consistency
    // between the timestamp and the counters.
    state: Mutex<InnerState>,
}

#[derive(Debug)]
struct InnerState {
    last_timestamp: u128,
    // Map of Context -> Current Counter
    counters: HashMap<u128, u128>,
}

impl InMemoryBackend {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            state: Mutex::new(InnerState {
                last_timestamp: 0,
                counters: HashMap::new(),
            }),
        })
    }
}

impl super::StateBackend for InMemoryBackend {
    fn next_sequence(&self, timestamp: u128, context: u128) -> Result<u128, VeigoIdError> {
        let mut state = self.state.lock().unwrap();

        if timestamp > state.last_timestamp {
            // New second started: Reset everything
            state.last_timestamp = timestamp;
            state.counters.clear();

            // Start this context at 0
            state.counters.insert(context, 0);
            return Ok(0);
        } else if timestamp == state.last_timestamp {
            // Same second: Increment counter
            let counter = state.counters.entry(context).or_insert(0);
            *counter += 1;
            return Ok(*counter);
        } else {
            // Clock moved backwards (timestamp < state.last_timestamp)
            return Err(VeigoIdError::ClockSkew {
                now: timestamp,
                last: state.last_timestamp,
            });
        }
    }
}

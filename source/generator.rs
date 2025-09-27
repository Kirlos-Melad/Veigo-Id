use std::sync::Arc;
use std::time::SystemTime;

use crate::{backend::StateBackend, config::VeigoConfig, errors::VeigoIdError, id::VeigoId};

#[derive(Debug)]
pub struct VeigoIdParts {
    pub timestamp: u128,
    pub context: u128,
    pub counter: u128,
}

#[derive(Debug, Clone)]
pub struct VeigoIdGenerator {
    backend: Arc<dyn StateBackend>,
    config: VeigoConfig,
}

impl VeigoIdGenerator {
    pub fn new(config: VeigoConfig, backend: Arc<dyn StateBackend>) -> Result<Self, VeigoIdError> {
        config.validate()?;
        Ok(Self { backend, config })
    }

    fn current_seconds(&self) -> u128 {
        SystemTime::now()
            .duration_since(self.config.epoch)
            .unwrap()
            .as_secs() as u128
    }

    pub fn generate(&self, context: u128) -> Result<VeigoId, VeigoIdError> {
        if context > self.config.max_context() {
            return Err(VeigoIdError::FieldOverflow {
                field: "context",
                value: context,
                max: self.config.max_context(),
            });
        }

        let ts = self.current_seconds();
        if ts > self.config.max_timestamp() {
            return Err(VeigoIdError::FieldOverflow {
                field: "timestamp",
                value: ts,
                max: self.config.max_timestamp(),
            });
        }

        let last_ts = self.backend.get_last_timestamp();
        if ts < last_ts {
            return Err(VeigoIdError::ClockSkew {
                now: ts,
                last: last_ts,
            });
        }

        if ts != last_ts {
            self.backend.clear_counters();
            self.backend.set_last_timestamp(ts);
        }

        let mut counter = self.backend.get_counter(context);
        if counter > self.config.max_counter() {
            return Err(VeigoIdError::FieldOverflow {
                field: "counter",
                value: counter,
                max: self.config.max_counter(),
            });
        }

        let id = (ts << (self.config.context_bits + self.config.counter_bits))
            | (context << self.config.counter_bits)
            | counter;

        counter += 1;
        self.backend.set_counter(context, counter);

        Ok(VeigoId::from(id))
    }

    pub fn decode(&self, id: VeigoId) -> VeigoIdParts {
        let id: u128 = id.into();
        let counter_mask = (1u128 << self.config.counter_bits) - 1;
        let context_mask = (1u128 << self.config.context_bits) - 1;

        let counter = id & counter_mask;
        let context = (id >> self.config.counter_bits) & context_mask;
        let timestamp = id >> (self.config.context_bits + self.config.counter_bits);

        VeigoIdParts {
            timestamp,
            context,
            counter,
        }
    }
}

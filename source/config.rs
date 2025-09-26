use chrono::DateTime;
use std::time::SystemTime;

use crate::errors::VeigoIdError;

#[derive(Debug, Clone)]
pub struct VeigoConfig {
    pub ts_bits: u8,
    pub context_bits: u8,
    pub counter_bits: u8,
    pub epoch: SystemTime,
}

impl Default for VeigoConfig {
    fn default() -> Self {
        let epoch = DateTime::from_timestamp(1735689600, 0).unwrap().into(); // 2025-01-01T00:00:00Z
        Self {
            ts_bits: 41,
            context_bits: 68,
            counter_bits: 18,
            epoch,
        }
    }
}

impl VeigoConfig {
    pub fn validate(&self) -> Result<(), VeigoIdError> {
        let total = 1 + self.ts_bits as u16 + self.context_bits as u16 + self.counter_bits as u16;
        if total > 128 {
            return Err(VeigoIdError::InvalidConfig(
                "total bits must be ≤ 128 including sign",
            ));
        }
        Ok(())
    }

    pub fn max_ts(&self) -> u128 {
        (1u128 << self.ts_bits) - 1
    }
    pub fn max_context(&self) -> u128 {
        (1u128 << self.context_bits) - 1
    }
    pub fn max_counter(&self) -> u128 {
        (1u128 << self.counter_bits) - 1
    }
}


use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::errors::VeigoIdError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Field {
    Timestamp { bits: u8 },
    Context { bits: u8 },
    Counter { bits: u8 },
    NodeId { bits: u8 },
}

impl Field {
    pub fn max_value(&self) -> u128 {
        match *self {
            Field::Timestamp { bits }
            | Field::Context { bits }
            | Field::Counter { bits }
            | Field::NodeId { bits } => (1u128 << bits) - 1,
        }
    }

    pub fn bits(&self) -> u8 {
        match *self {
            Field::Timestamp { bits }
            | Field::Context { bits }
            | Field::Counter { bits }
            | Field::NodeId { bits } => bits,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VeigoConfig {
    pub epoch: SystemTime,
    pub layout: [Field; 4],
}

impl Default for VeigoConfig {
    fn default() -> Self {
        // 2025-01-01T00:00:00Z
        let epoch = DateTime::from_timestamp(1735689600, 0).unwrap().into();
        let layout = [
            Field::Timestamp { bits: 41 },
            Field::Context { bits: 60 },
            Field::Counter { bits: 16 },
            Field::NodeId { bits: 10 },
        ];
        Self { epoch, layout }
    }
}

impl VeigoConfig {
    pub fn validate(&self) -> Result<(), VeigoIdError> {
        let total_bits: u16 = self
            .layout
            .iter()
            .map(|f| match f {
                Field::Timestamp { bits } => *bits as u16,
                Field::Context { bits } => *bits as u16,
                Field::Counter { bits } => *bits as u16,
                Field::NodeId { bits } => *bits as u16,
            })
            .sum();
        if total_bits > 127 {
            return Err(VeigoIdError::InvalidConfiguration(
                "total bits must be ≤ 127 excluding sign bit",
            ));
        }

        let mut seen = (false, false, false, false); // ts, ctx, ctr, nid
        for f in &self.layout {
            match f {
                Field::Timestamp { .. } if !seen.0 => seen.0 = true,
                Field::Context { .. } if !seen.1 => seen.1 = true,
                Field::Counter { .. } if !seen.2 => seen.2 = true,
                Field::NodeId { .. } if !seen.3 => seen.3 = true,
                _ => {
                    return Err(VeigoIdError::InvalidConfiguration(
                        "layout must contain exactly one of each field",
                    ));
                }
            }
        }

        Ok(())
    }
}

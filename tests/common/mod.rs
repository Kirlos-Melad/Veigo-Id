use std::time::SystemTime;

use veigo_id::{Field, VeigoConfig};

// Helper to create a specific config for testing limits
pub fn create_test_config(ctx_bits: u8, node_bits: u8, ctr_bits: u8) -> VeigoConfig {
    let used = ctx_bits + node_bits + ctr_bits;
    // Remaining bits go to timestamp to ensure total <= 127
    let ts_bits = 126 - used;

    VeigoConfig {
        epoch: SystemTime::UNIX_EPOCH,
        layout: [
            Field::Timestamp { bits: ts_bits },
            Field::Context { bits: ctx_bits },
            Field::NodeId { bits: node_bits },
            Field::Counter { bits: ctr_bits },
        ],
    }
}

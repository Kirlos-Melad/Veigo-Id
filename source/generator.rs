use std::sync::Arc;
use std::time::SystemTime;

use crate::{
    backend::StateBackend,
    config::{Field, VeigoConfig},
    errors::VeigoIdError,
    id::VeigoId,
};

#[derive(Debug)]
pub struct VeigoIdParts {
    pub timestamp: u128,
    pub context: u128,
    pub counter: u128,
    pub node_id: u128,
}

/// A helper struct to store pre-calculated masks and shifts.
/// This prevents recalculating bit logic on every ID generation.
#[derive(Debug, Clone)]
struct LayoutCache {
    // Shifts (How far left to move the bits)
    ts_shift: u8,
    ctx_shift: u8,
    ctr_shift: u8,
    node_shift: u8,

    // Max Values (For validation)
    max_ts: u128,
    max_ctx: u128,
    max_ctr: u128,
    // max_node is not needed for runtime checks since node_id is fixed in struct

    // Masks (For decoding)
    ts_mask: u128,
    ctx_mask: u128,
    ctr_mask: u128,
    node_mask: u128,
}

/// # ⚠️ Internal Component
///
/// This is the low-level generator struct.
///
/// **Most users should NOT use this directly.**
///
/// Instead, use the global singleton approach:
/// 1. Call [`crate::configure`] at startup.
/// 2. Call [`crate::generate`] to create IDs.
///
/// Use this struct directly only if you need:
/// - To write Unit/Integration tests.
/// - To manage multiple distinct generators in the same application.
/// - Dependency Injection.
#[derive(Debug, Clone)]
pub struct VeigoIdGenerator {
    backend: Arc<dyn StateBackend>,
    config: VeigoConfig,
    cache: LayoutCache,
    /// The Node ID is usually constant for the lifecycle of the generator instance
    node_id: u128,
}

impl VeigoIdGenerator {
    /// Creates a new isolated generator instance.
    ///
    /// **Note:** Prefer using [`crate::configure`] for standard applications.
    pub fn new(
        config: VeigoConfig,
        backend: Arc<dyn StateBackend>,
        node_id: u128,
    ) -> Result<Self, VeigoIdError> {
        config.validate()?;

        // 1. Build the Layout Cache
        // We calculate this once so 'generate' is extremely fast.
        let mut shift_accumulator = 0;
        let mut cache = LayoutCache {
            ts_shift: 0,
            ctx_shift: 0,
            ctr_shift: 0,
            node_shift: 0,
            max_ts: 0,
            max_ctx: 0,
            max_ctr: 0,
            ts_mask: 0,
            ctx_mask: 0,
            ctr_mask: 0,
            node_mask: 0,
        };

        // We iterate in reverse (LSB to MSB) to calculate shifts correctly
        for field in config.layout.iter().rev() {
            let bits = field.bits();
            let max_val = field.max_value();
            let mask = max_val; // Max value is effectively the mask (e.g., 1111)

            match field {
                Field::Timestamp { .. } => {
                    cache.ts_shift = shift_accumulator;
                    cache.max_ts = max_val;
                    cache.ts_mask = mask;
                }
                Field::Context { .. } => {
                    cache.ctx_shift = shift_accumulator;
                    cache.max_ctx = max_val;
                    cache.ctx_mask = mask;
                }
                Field::Counter { .. } => {
                    cache.ctr_shift = shift_accumulator;
                    cache.max_ctr = max_val;
                    cache.ctr_mask = mask;
                }
                Field::NodeId { .. } => {
                    cache.node_shift = shift_accumulator;
                    // Validate NodeID immediately on startup
                    if node_id > max_val {
                        return Err(VeigoIdError::FieldOverflow {
                            field: "node_id initialization",
                            value: node_id,
                            max: max_val,
                        });
                    }
                    cache.node_mask = mask;
                }
            }
            shift_accumulator += bits;
        }

        Ok(Self {
            backend,
            config,
            cache,
            node_id: node_id,
        })
    }

    fn current_seconds(&self) -> u128 {
        SystemTime::now()
            .duration_since(self.config.epoch)
            .unwrap()
            .as_secs() as u128
    }

    /// Helper for validation to reduce code duplication
    #[inline(always)]
    fn check_overflow(
        &self,
        name: &'static str,
        value: u128,
        max: u128,
    ) -> Result<(), VeigoIdError> {
        if value > max {
            Err(VeigoIdError::FieldOverflow {
                field: name,
                value,
                max,
            })
        } else {
            Ok(())
        }
    }

    pub fn generate(&self, context: u128) -> Result<VeigoId, VeigoIdError> {
        // 1. Validate Context (using cached max value)
        self.check_overflow("context", context, self.cache.max_ctx)?;

        // 2. Get Time
        let ts = self.current_seconds();
        self.check_overflow("timestamp", ts, self.cache.max_ts)?;

        // 3. Atomic State Update
        // The backend now handles locking, time checks, and incrementing safely.
        let counter = self.backend.next_sequence(ts, context)?;

        // 4. Validate Counter
        self.check_overflow("counter", counter, self.cache.max_ctr)?;

        // 5. Construct ID (Shift logic remains the same)
        let id = (ts << self.cache.ts_shift)
            | (context << self.cache.ctx_shift)
            | (self.node_id << self.cache.node_shift)
            | (counter << self.cache.ctr_shift);

        Ok(VeigoId::from(id))
    }

    pub fn decode(&self, id: VeigoId) -> VeigoIdParts {
        let raw: u128 = id.into();

        // Decoding is now generic based on the cached masks and shifts
        VeigoIdParts {
            timestamp: (raw >> self.cache.ts_shift) & self.cache.ts_mask,
            context: (raw >> self.cache.ctx_shift) & self.cache.ctx_mask,
            counter: (raw >> self.cache.ctr_shift) & self.cache.ctr_mask,
            node_id: (raw >> self.cache.node_shift) & self.cache.node_mask,
        }
    }
}

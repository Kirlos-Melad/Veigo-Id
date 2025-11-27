use crate::VeigoIdError;

pub mod in_memory;

pub trait StateBackend: Send + Sync + std::fmt::Debug + 'static {
    /// Atomically retrieves the next sequence number for a specific timestamp and context.
    ///
    /// Logic inside the backend:
    /// 1. If `timestamp` > `stored_timestamp`: Update stored_time, reset counters, return 0.
    /// 2. If `timestamp` == `stored_timestamp`: Increment counter for context, return value.
    /// 3. If `timestamp` < `stored_timestamp`: Return Error (Clock moved backwards).
    fn next_sequence(&self, timestamp: u128, context: u128) -> Result<u128, VeigoIdError>;
}

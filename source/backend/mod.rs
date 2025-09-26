pub mod memory_backend;

pub trait StateBackend: Send + Sync + 'static {
    fn get_last_timestamp(&self) -> u128;
    fn set_last_timestamp(&self, ts: u128);

    fn get_counter(&self, context: u128) -> u128;
    fn set_counter(&self, context: u128, counter: u128);
    fn clear_counters(&self);
}

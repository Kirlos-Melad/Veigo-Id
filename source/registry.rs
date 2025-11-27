use std::sync::{Arc, OnceLock};

use crate::{
    backend::StateBackend, config::VeigoConfig, errors::VeigoIdError, generator::VeigoIdGenerator,
    id::VeigoId,
};

// 1. Use OnceLock.
// It guarantees the value is set exactly once and provides wait-free access afterwards.
static GLOBAL_GENERATOR: OnceLock<VeigoIdGenerator> = OnceLock::new();

/// Initializes the global generator.
/// This must be called once at application startup.
pub fn configure(
    config: Option<VeigoConfig>,
    backend: Arc<dyn StateBackend>,
    node_id: u128, // <--- ADDED: Required by generator::new()
) -> Result<(), VeigoIdError> {
    // Check if already configured to avoid doing work unnecessarily
    if GLOBAL_GENERATOR.get().is_some() {
        return Err(VeigoIdError::AlreadyConfigured);
    }

    let cfg = config.unwrap_or_default();

    // We create the instance locally first
    let generator = VeigoIdGenerator::new(cfg, backend, node_id)?;

    // .set() returns Result<(), T> where T is the value we tried to set
    // if it failed (meaning it was already set).
    GLOBAL_GENERATOR
        .set(generator)
        .map_err(|_| VeigoIdError::AlreadyConfigured)
}

/// Access the global generator instance.
/// Returns a reference so we don't need to clone Arcs unnecessarily.
pub fn get_global() -> Result<&'static VeigoIdGenerator, VeigoIdError> {
    GLOBAL_GENERATOR.get().ok_or(VeigoIdError::NotConfigured)
}

/// Helper function to generate an ID directly from the global instance
/// Usage: veigo::generate(context_id)?
pub fn generate(context: u128) -> Result<VeigoId, VeigoIdError> {
    get_global()?.generate(context)
}


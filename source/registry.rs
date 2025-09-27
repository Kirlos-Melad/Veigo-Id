use crate::backend::StateBackend;
use crate::{config::VeigoConfig, errors::VeigoIdError, generator::VeigoIdGenerator};
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};

static GLOBAL_GENERATOR: Lazy<RwLock<Option<Arc<VeigoIdGenerator>>>> =
    Lazy::new(|| RwLock::new(None));

pub fn configure(
    config: Option<VeigoConfig>,
    backend: Arc<dyn StateBackend>,
) -> Result<(), VeigoIdError> {
    let cfg = config.unwrap_or_default();
    cfg.validate()?;

    let vgen = Arc::new(VeigoIdGenerator::new(cfg, backend)?);

    let mut global = GLOBAL_GENERATOR
        .write()
        .map_err(|_| VeigoIdError::Poisoned)?;

    if global.is_some() {
        return Err(VeigoIdError::AlreadyConfigured);
    }

    *global = Some(vgen);
    Ok(())
}

pub fn get_global() -> Result<Arc<VeigoIdGenerator>, VeigoIdError> {
    GLOBAL_GENERATOR
        .read()
        .map_err(|_| VeigoIdError::Poisoned)?
        .clone()
        .ok_or(VeigoIdError::NotConfigured)
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum VeigoIdError {
    #[error("system clock moved backwards: now={now}, last={last}")]
    ClockSkew { now: u128, last: u128 },

    #[error("field overflow: {field} value={value}, max={max}")]
    FieldOverflow {
        field: &'static str,
        value: u128,
        max: u128,
    },

    #[error("Generator already configured")]
    AlreadyConfigured,

    #[error("Generator not configured")]
    NotConfigured,

    #[error("Generator lock poisoned")]
    Poisoned,

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(&'static str),

    #[error("Backend error: {0}")]
    Backend(String),
}

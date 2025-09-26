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

    #[error("invalid configuration: {0}")]
    InvalidConfig(&'static str),
}

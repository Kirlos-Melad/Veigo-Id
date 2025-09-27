use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VeigoId(pub u128);

impl fmt::Display for VeigoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u128> for VeigoId {
    fn from(value: u128) -> Self {
        VeigoId(value)
    }
}

impl From<VeigoId> for u128 {
    fn from(id: VeigoId) -> u128 {
        id.0
    }
}

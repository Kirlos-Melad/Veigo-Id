use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{VeigoIdError, generator::VeigoIdParts, registry};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VeigoId(pub u128);

impl VeigoId {
    pub fn new(context: u128) -> Result<Self, VeigoIdError> {
        let vgen = registry::get_global()?;
        vgen.generate(context)
    }

    pub fn decode(&self) -> Result<VeigoIdParts, VeigoIdError> {
        let vgen = registry::get_global()?;
        Ok(vgen.decode(*self))
    }
}

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

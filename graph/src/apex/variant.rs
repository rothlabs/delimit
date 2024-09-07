use anyhow::anyhow;

use super::*;

impl Apex {
    pub fn string(self) -> Result<Hub<String>> {
        match self {
            Self::String(x) => Ok(x),
            _ => Err(anyhow!("not a string"))?
        }
    }
}
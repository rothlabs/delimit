use anyhow::anyhow;

use super::*;

impl Apex {
    pub fn string(self) -> Result<Hub<String>> {
        match self {
            Self::String(x) => Ok(x),
            _ => Err(anyhow!("not a string"))?,
        }
    }
    pub fn vec_f32(self) -> Result<Hub<Vec<f32>>> {
        match self {
            Self::Vf32(x) => Ok(x),
            _ => Err(anyhow!("not Vf32"))?,
        }
    }
}

use anyhow::anyhow;
use super::*;

mod convert;

pub enum View<'a> {
    String(&'a mut Hub<String>),
    U8(&'a mut Hub<u8>),
}

impl View<'_> {
    pub fn apex(&self) -> Apex {
        match self {
            Self::String(x) => Apex::String((*x).clone()),
            Self::U8(x) => Apex::U8((*x).clone()),
        }
    }
    pub fn set(self, apex: Apex) -> Result<Self> {
        match self {
            Self::String(x) => if let Apex::String(y) = apex {
                *x = y;
                return Ok(Self::String(x));
            },
            Self::U8(x) => if let Apex::U8(y) = apex {
                *x = y; 
                return Ok(Self::U8(x));
            },
        };
        Err(anyhow!("view and apex types do not match"))?
    }
    pub fn tray_hash(&self) -> Option<u64> {
        match self {
            Self::String(x) => x.tray_hash(),
            Self::U8(x) => x.tray_hash(),
        }
    }
    pub fn tray_path(&self) -> Option<&Path> {
        match self {
            Self::String(x) => x.tray_path(),
            Self::U8(x) => x.tray_path(),
        }
    }
}
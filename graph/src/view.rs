use super::*;
use anyhow::anyhow;

mod convert;

pub enum View<'a> {
    Void(&'a mut Hub<()>),
    String(&'a mut Hub<String>),
    U8(&'a mut Hub<u8>),
    Vu8(&'a mut Hub<Vec<u8>>),
    Vu16(&'a mut Hub<Vec<u16>>),
    Vf32(&'a mut Hub<Vf32>),
}

impl View<'_> {
    pub fn apex(self) -> Apex {
        match self {
            Self::Void(x) => Apex::Void(x.clone()),
            Self::String(x) => Apex::String(x.clone()),
            Self::U8(x) => Apex::U8(x.clone()),
            Self::Vu8(x) => Apex::Vu8(x.clone()),
            Self::Vu16(x) => Apex::Vu16(x.clone()),
            Self::Vf32(x) => Apex::Vf32(x.clone()),
        }
    }
    pub fn path(&self) -> Option<&Path> {
        match self {
            Self::Void(x) => x.path(),
            Self::String(x) => x.path(),
            Self::U8(x) => x.path(),
            Self::Vu8(x) => x.path(),
            Self::Vu16(x) => x.path(),
            Self::Vf32(x) => x.path(),
        }
    }
    pub fn backed(&self, back: &Back) -> Result<Apex> {
        let apex = match self {
            Self::Void(x) => Apex::Void(x.backed(back)?),
            Self::String(x) => Apex::String(x.backed(back)?),
            Self::U8(x) => Apex::U8(x.backed(back)?),
            Self::Vu8(x) => Apex::Vu8(x.backed(back)?),
            Self::Vu16(x) => Apex::Vu16(x.backed(back)?),
            Self::Vf32(x) => Apex::Vf32(x.backed(back)?),
        };
        Ok(apex)
    }
    pub fn set(self, apex: Apex) -> Result<Self> {
        match self {
            Self::Void(x) => {
                if let Apex::Void(y) = apex {
                    *x = y;
                    return Ok(Self::Void(x));
                }
            }
            Self::String(x) => {
                if let Apex::String(y) = apex {
                    *x = y;
                    return Ok(Self::String(x));
                }
            }
            Self::U8(x) => {
                if let Apex::U8(y) = apex {
                    *x = y;
                    return Ok(Self::U8(x));
                }
            }
            Self::Vu8(x) => {
                if let Apex::Vu8(y) = apex {
                    *x = y;
                    return Ok(Self::Vu8(x));
                }
            }
            Self::Vu16(x) => {
                if let Apex::Vu16(y) = apex {
                    *x = y;
                    return Ok(Self::Vu16(x));
                }
            }
            Self::Vf32(x) => {
                if let Apex::Vf32(y) = apex {
                    *x = y;
                    return Ok(Self::Vf32(x));
                }
            }
        };
        Err(anyhow!("view and apex types do not match"))?
    }
}

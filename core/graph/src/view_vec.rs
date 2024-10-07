use super::*;
use anyhow::anyhow;

mod convert;

pub enum ViewVec<'a> {
    String(&'a mut Vec<Hub<String>>),
    F64(&'a mut Vec<Hub<f64>>),
}

macro_rules! ImplViewVec {
    ($($variant:ident)*) => {
        impl<'a> ViewVec<'a> {
            pub fn len(&self) -> usize {
                match self {
                    $(ViewVec::$variant(x) => x.len(),)*
                }
            }
        }
    };
}

ImplViewVec!(String F64);

impl<'a> ViewVec<'a> {
    // fn len(&self) -> usize {
    //     match self {
    //         Self::String(x) => x.len(),
    //         Self::F64(x) => x.len(),
    //     }
    // }
    /// Get `Vec<View>`
    pub fn views(self) -> Vec<View<'a>> {
        let mut views = vec![];
        match self {
            Self::String(hubs) => {
                for hub in hubs {
                    views.push(hub.into());
                }
            },
            Self::F64(hubs) => {
                for hub in hubs {
                    views.push(hub.into());
                }
            }
        };
        views
    }
    pub fn set(self, i: usize, apex: Apex) -> Result<Self> {
        if i >= self.len() {
            return Err(anyhow!("index out of bounds"))?;
        }
        match self {
            Self::String(x) => {
                if let Apex::String(y) = apex {
                    x[i] = y;
                    return Ok(Self::String(x));
                }
            },
            Self::F64(x) => {
                if let Apex::F64(y) = apex {
                    x[i] = y;
                    return Ok(Self::F64(x));
                }
            }
        };
        Err(anyhow!("view and apex types do not match"))?
    }
    pub fn apex(&self, i: usize) -> Result<Apex> {
        if i >= self.len() {
            return Err(anyhow!("index out of bounds"))?;
        }
        let apex = match self {
            Self::String(x) => Apex::String(x[i].clone()),
            Self::F64(x) => Apex::F64(x[i].clone()),
        };
        Ok(apex)
    }
    pub fn all(&self) -> Vec<Apex> {
        let mut apexes = vec![];
        match self {
            Self::String(x) => {
                for hub in x.iter() {
                    apexes.push(Apex::String(hub.clone()))
                }
            }
            Self::F64(x) => {
                for hub in x.iter() {
                    apexes.push(Apex::F64(hub.clone()))
                }
            }
        };
        apexes
    }
}

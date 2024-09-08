use super::*;
use anyhow::anyhow;

mod convert;

pub enum ViewVec<'a> {
    String(&'a mut Vec<Hub<String>>),
}

impl<'a> ViewVec<'a> {
    fn len(&self) -> usize {
        match self {
            Self::String(x) => x.len(),
        }
    }
    /// Get `Vec<View>`
    pub fn views(self) -> Vec<View<'a>> {
        let mut views = vec![];
        match self {
            Self::String(x) => {
                for hub in x {
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
        };
        apexes
    }
}

use anyhow::anyhow;

use super::*;

pub enum VecView<'a> {
    VecString(&'a mut Vec<Hub<String>>),
}

impl VecView<'_> {
    pub fn len(&self) -> usize {
        match self {
            Self::VecString(x) => x.len(),
        }
    }
    pub fn get(&mut self, i: usize) -> Result<View> {
        if i >= self.len() {
            return Err(anyhow!("index out of bounds"))?;
        }
        let view = match self {
            Self::VecString(x) => (&mut x[0]).into()
        };
        Ok(view)
    }
    pub fn set(&mut self, i: usize, apex: Apex) -> Result<()> {
        if i >= self.len() {
            return Err(anyhow!("index out of bounds"))?;
        }
        let mut ok = false;
        match self {
            Self::VecString(x) => if let Apex::String(y) = apex {
                x[i] = y; 
                ok = true;
            },
        };
        if ok {
            Ok(())
        } else {
            Err(anyhow!("view_vec: set failed, apex type does not match"))?
        }
    }
    pub fn apex(&self, i: usize) -> Result<Apex> {
        if i >= self.len() {
            return Err(anyhow!("index out of bounds"))?;
        }
        let apex = match self {
            Self::VecString(x) => Apex::String(x[i].clone()),
        };
        Ok(apex)
    }
    pub fn all(&self) -> Vec<Apex> {
        let mut apexes = vec![];
        match self {
            Self::VecString(x) => {
                for hub in x.iter() {
                    apexes.push(Apex::String(hub.clone()))
                } 
            },
        };
        apexes
    }
}
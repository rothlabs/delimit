use anyhow::anyhow;
use super::*;

mod convert;

pub enum View<'a> {
    String(&'a mut Hub<String>),
    U8(&'a mut Hub<u8>),

    VecString(&'a mut Vec<Hub<String>>),
}

impl View<'_> {
    pub fn len(&self) -> Result<usize> {
        let len = match self {
            Self::VecString(x) => x.len(),
            _ => Err(anyhow!("view len"))?
        };
        Ok(len)
    }
    pub fn all(&self) -> Result<Vec<Apex>> {
        let mut apexes = vec![];
        match self {
            Self::VecString(x) => {
                for apex in x.iter() {
                    apexes.push(Apex::String(apex.clone()))
                } 
            },
            _ => Err(anyhow!("view all"))?
        };
        Ok(apexes)
    }
    pub fn apex(&self) -> Apex {
        match self {
            Self::String(x) => Apex::String((*x).clone()),
            Self::U8(x) => Apex::U8((*x).clone()),
            _ => panic!("view apex"),
        }
    }
    pub fn apex_at(self, i: usize) -> Result<Apex> {
        let apex = match self {
            Self::VecString(x) => Apex::String(x[i].clone()),
            _ => Err(anyhow!("view apex_at"))?
        };
        Ok(apex)
    }
    pub fn set(self, apex: Apex) -> Result<()> {
        match self {
            Self::String(x) => if let Apex::String(y) = apex {*x = y;},
            Self::U8(x) => if let Apex::U8(y) = apex {*x = y;},
            _ => Err(anyhow!("view set"))?
        };
        Ok(())
    }
    pub fn set_at(self, i: usize, apex: Apex) -> Result<()> {
        match self {
            Self::VecString(x) => if let Apex::String(y) = apex {x[i] = y;},
            _ => Err(anyhow!("view set_at"))?
        };
        Ok(())
    }
    pub fn tray_hash(&self) -> Option<u64> {
        match self {
            Self::String(x) => x.tray_hash(),
            Self::U8(x) => x.tray_hash(),
            _ => panic!("view tray_hash"),
        }
    }
    pub fn tray_path(&self) -> Option<&Path> {
        match self {
            Self::String(x) => x.tray_path(),
            Self::U8(x) => x.tray_path(),
            _ => panic!("view tray_path"),
        }
    }
}
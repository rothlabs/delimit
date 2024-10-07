use super::*;
use anyhow::anyhow;

mod convert;

pub enum ViewVec<'a> {
    Void(&'a mut Vec<Hub<()>>),
    String(&'a mut Vec<Hub<String>>),
    F64(&'a mut Vec<Hub<f64>>),
}

macro_rules! ImplViewVec {
    ($($Variant:ident)*) => {
        impl<'a> ViewVec<'a> {
            pub fn len(&self) -> usize {
                match self {
                    $(Self::$Variant(x) => x.len(),)*
                }
            }
            pub fn views(self) -> Vec<View<'a>> {
                let mut views = vec![];
                match self {
                    $(Self::$Variant(hubs) => {
                        for hub in hubs {
                            views.push(hub.into());
                        }
                    },)*
                };
                views
            }
            pub fn set(self, i: usize, apex: Apex) -> Result<Self> {
                if i >= self.len() {
                    return Err(anyhow!("index out of bounds"))?;
                }
                match self {
                    $(Self::$Variant(x) => {
                        if let Apex::$Variant(y) = apex {
                            x[i] = y;
                            return Ok(Self::$Variant(x));
                        }
                    },)*
                };
                Err(anyhow!("view and apex types do not match"))?
            }
            pub fn apex(&self, i: usize) -> Result<Apex> {
                if i >= self.len() {
                    return Err(anyhow!("index out of bounds"))?;
                }
                let apex = match self {
                    $(Self::$Variant(x) => Apex::$Variant(x[i].clone()),)*
                };
                Ok(apex)
            }
            pub fn all(&self) -> Vec<Apex> {
                let mut apexes = vec![];
                match self {
                    $(Self::$Variant(x) => {
                        for hub in x.iter() {
                            apexes.push(Apex::$Variant(hub.clone()))
                        }
                    },)*
                };
                apexes
            }
        }
    };
}

ImplViewVec!(Void String F64);
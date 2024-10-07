use super::*;
use anyhow::anyhow;

mod convert;

macro_rules! ImplViewVec {
    ($($Variant:ident $type_:ty)*) => {

        pub enum View<'a> {
            $($Variant(&'a mut Hub<$type_>),)*
        }

        $(impl<'a> From<&'a mut Hub<$type_>> for View<'a> {
            fn from(x: &'a mut Hub<$type_>) -> Self {
                Self::$Variant(x)
            }
        })*

        impl View<'_> {
            pub fn apex(self) -> Apex {
                match self {
                    $(Self::$Variant(x) => Apex::$Variant(x.clone()),)*
                }
            }
            pub fn path(&self) -> Option<&Path> {
                // TODO: use macro_rules to reduce this
                match self {
                    $(Self::$Variant(x) => x.path(),)*
                }
            }
            pub fn backed(&self, back: &Back) -> Result<Apex> {
                Ok(match self {
                    $(Self::$Variant(x) => Apex::$Variant(x.backed(back)?),)*
                })
            }
            pub fn set(self, apex: Apex) -> Result<Self> {
                // TODO: use macro_rules to reduce this
                match self {
                    $(Self::$Variant(x) => {
                        if let Apex::$Variant(y) = apex {
                            *x = y;
                            return Ok(Self::$Variant(x));
                        }
                    })*
                };
                Err(anyhow!("view and apex types do not match"))?
            }
        }

        pub enum ViewVec<'a> {
            $($Variant(&'a mut Vec<Hub<$type_>>),)*
        }

        $(impl<'a> From<&'a mut Vec<Hub<$type_>>> for ViewVec<'a> {
            fn from(x: &'a mut Vec<Hub<$type_>>) -> Self {
                Self::$Variant(x)
            }
        })*

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

ImplViewVec!(
    Void ()
    String String
    U8 u8
    I32 i32
    F64 f64
    Vu8 Vec<u8>
    Vu16 Vec<u16>
    Vf32 Vec<f32>
    Vf64 Vec<f64>
);
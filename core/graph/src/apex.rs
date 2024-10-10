use super::*;

mod variant;

impl Default for Apex {
    fn default() -> Self {
        Self::String("".leaf().hub())
    }
}

impl Apex {
    pub async fn hydrate(&self) -> Result<()> {
        let space = Space::new(vec![], self)?;
        let mut ring = Ring::new();
        self.saturate(&space, &space, &mut ring)?;
        ring.react().await
    }
    fn saturate(&self, world: &Space, local: &Space, ring: &mut Ring) -> Result<()> {
        ring.extend(self.transient_set(&mut Scope {
            world,
            local,
            back: None,
        })?);
        for spaces in local.map.values() {
            for space in spaces {
                space.apex.saturate(world, space, ring)?;
            }
        }
        Ok(())
    }
}

pub trait Poll {
    fn poll(&self) -> impl Future<Output = Result<()>> + IsSend;
}

impl Poll for Vec<Apex> {
    async fn poll(&self) -> Result<()> {
        for hub in self {
            hub.poll().await?;
        }
        Ok(())
    }
}

pub trait DealItem {
    fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()>;
}

impl DealItem for Apex {
    fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
        deal.one(key, View::Apex(self))
    }
}

impl DealItem for Vec<Apex> {
    fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
        deal.vec(key, ViewVec::Apex(self))
    }
}

// pub trait DealItem2 {
//     fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()>;
// }

// impl DealItem2 for Vec<Hub<T>> {
//     fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
//         deal.vec(key, self.into())
//     }
// }

impl From<&str> for Apex {
    fn from(value: &str) -> Self {
        Apex::String(value.into())
    }
}

// impl From<Link<dyn 'static + Employ<Base = ()>>> for Apex
// // where
// //     E: 'static + Employ<Base = ()>,
// {
//     fn from(node: Link<dyn 'static + Employ<Base = ()>>) -> Self {
//         Apex::Void(node.into())
//     }
// }

macro_rules! ImplViewVec {
    ($($Variant:ident $type_:ty)*) => {

        $(impl Hub<$type_> {
            pub fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
                deal.one(key, self.into())
            }
        })*

        $(impl DealItem for Option<Hub<$type_>> {
            fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
                if let Some(hub) = self {
                    deal.one(key, hub.into())
                } else {
                    Ok(())
                }
            }
        })*

        $(impl DealItem for Vec<Hub<$type_>> {
            fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
                deal.vec(key, self.into())
            }
        })*

        ////////

        #[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
        pub enum Apex {
            $($Variant(Hub<$type_>),)*
        }

        // $(impl<E> From<Link<E>> for Apex
        // where
        //     E: 'static + Employ<Base = $type_>,
        // {
        //     fn from(node: Link<E>) -> Self {
        //         Apex::$Variant(node.wing().into())
        //     }
        // })*

        impl From<&Apex> for Apex {
            fn from(value: &Apex) -> Self {
                match value {
                    $(Apex::$Variant(hub) => Apex::$Variant(hub.clone()),)*
                }
            }
        }

        $(impl From<Hub<$type_>> for Apex {
            fn from(value: Hub<$type_>) -> Self {
                Apex::$Variant(value)
            }
        })*

        $(impl From<&Hub<$type_>> for Apex {
            fn from(value: &Hub<$type_>) -> Self {
                Apex::$Variant(value.clone())
            }
        })*

        $(impl From<$type_> for Apex {
            fn from(value: $type_) -> Self {
                Apex::$Variant(value.into())
            }
        })*

        $(impl From<Wing<$type_>> for Apex {
            fn from(value: Wing<$type_>) -> Self {
                Apex::$Variant(value.into())
            }
        })*

        impl HashGraph for Apex {
            fn hash_graph<H: Hasher>(&self, state: &mut H) {
                match self {
                    $(Self::$Variant(x) => x.hash_graph(state),)*
                }
            }
        }

        impl Backed for Apex {
            fn backed(&self, back: &Back) -> Result<Self> {
                let apex = match self {
                    $(Self::$Variant(x) => Self::$Variant(x.backed(back)?),)*
                };
                Ok(apex)
            }
        }

        impl Apex {
            pub async fn poll(&self) -> Result<()> {
                match self {
                    $(Self::$Variant(x) => x.poll().await,)*
                }
            }
            pub fn path(&self) -> Option<&Path> {
                match self {
                    $(Self::$Variant(x) => x.path(),)*
                }
            }
            pub fn get(&self, aim: impl Into<Aim>) -> Result<Apex> {
                match self {
                    $(Self::$Variant(x) => x.get(aim),)*
                }
            }
            pub async fn set(&self, aim: impl Into<Aim>, apex: impl Into<Apex>) -> Result<()> {
                match self {
                    $(Self::$Variant(x) => x.set(aim, apex).await,)*
                }
            }
            pub fn adapt_get(&self, deal: &mut dyn Deal) -> Result<()> {
                match self {
                    $(Self::$Variant(x) => x.adapt_get(deal),)*
                }
            }
            pub fn transient_set(&self, deal: &mut dyn Deal) -> Result<Ring> {
                match self {
                    $(Self::$Variant(x) => x.transient_set(deal),)*
                }
            }
            pub fn tray_path(&self) -> Option<&Path> {
                match self {
                    $(Self::$Variant(x) => x.path(),)*
                }
            }
            pub fn pathed(&self, path: impl Into<Path>) -> Self {
                match self {
                    $(Self::$Variant(x) => Self::$Variant(x.pathed(path)),)*
                }
            }
            pub fn imports(&self) -> Result<Vec<Import>> {
                match self {
                    $(Self::$Variant(x) => x.imports(),)*
                }
            }
            pub fn all(&self) -> Result<Vec<Apex>> {
                match self {
                    $(Self::$Variant(x) => x.all(),)*
                }
            }
            pub fn insert_in_lake(&self, lake: &mut Lake) -> Result<()> {
                match self {
                    $(Self::$Variant(x) => lake.insert_stem(x),)*
                }
            }
            pub fn grow_from_lake(&self, lake: &mut Lake, ring: &mut Ring) -> Result<()> {
                match self {
                    $(Self::$Variant(x) => lake.grow(x, ring),)*
                }
            }
        }

        pub enum View<'a> {
            Apex(&'a mut Apex),
            $($Variant(&'a mut Hub<$type_>),)*
        }

        impl<'a> From<&'a mut Apex> for View<'a> {
            fn from(value: &'a mut Apex) -> Self {
                match value {
                    $(Apex::$Variant(x) => View::$Variant(x),)*
                }
            }
        }

        $(impl<'a> From<&'a mut Hub<$type_>> for View<'a> {
            fn from(x: &'a mut Hub<$type_>) -> Self {
                Self::$Variant(x)
            }
        })*

        impl View<'_> {
            pub fn apex(self) -> Apex {
                match self {
                    Self::Apex(x) => x.clone(),
                    $(Self::$Variant(x) => Apex::$Variant(x.clone()),)*
                }
            }
            pub fn path(&self) -> Option<&Path> {
                match self {
                    Self::Apex(x) => x.path(),
                    $(Self::$Variant(x) => x.path(),)*
                }
            }
            pub fn backed(&self, back: &Back) -> Result<Apex> {
                Ok(match self {
                    Self::Apex(x) => x.backed(back)?,
                    $(Self::$Variant(x) => Apex::$Variant(x.backed(back)?),)*
                })
            }
            pub fn set(self, apex: Apex) -> Result<Self> {
                match self {
                    Self::Apex(x) => {
                        *x = apex;
                        return Ok(Self::Apex(x));
                    },
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
            Apex(&'a mut Vec<Apex>),
            $($Variant(&'a mut Vec<Hub<$type_>>),)*
        }

        $(impl<'a> From<&'a mut Vec<Hub<$type_>>> for ViewVec<'a> {
            fn from(x: &'a mut Vec<Hub<$type_>>) -> Self {
                Self::$Variant(x)
            }
        })*

        impl<'a> ViewVec<'a> {
            fn len(&self) -> usize {
                match self {
                    Self::Apex(x) => x.len(),
                    $(Self::$Variant(x) => x.len(),)*
                }
            }
            pub fn views(self) -> Vec<View<'a>> {
                let mut views = vec![];
                match self {
                    Self::Apex(apexes) => {
                        for apex in apexes {
                            views.push(View::Apex(apex));
                        }
                    } ,
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
                    Self::Apex(x) => {
                        x[i] = apex;
                        return Ok(Self::Apex(x));
                    },
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
                    Self::Apex(x) => x[i].clone(),
                    $(Self::$Variant(x) => Apex::$Variant(x[i].clone()),)*
                };
                Ok(apex)
            }
            pub fn all(&self) -> Vec<Apex> {
                let mut apexes = vec![];
                match self {
                    Self::Apex(x) => {
                        for apex in x.iter() {
                            apexes.push(apex.clone())
                        }
                    },
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
    U16 u16
    U32 u32
    U64 u64
    I8 i8
    I16 i16
    I32 i32
    F32 f32
    F64 f64
    Vu8 Vec<u8>
    Vu16 Vec<u16>
    Vu32 Vec<u32>
    Vi8 Vec<i8>
    Vi16 Vec<i16>
    Vi32 Vec<i32>
    Vf32 Vec<f32>
    Vf64 Vec<f64>
);

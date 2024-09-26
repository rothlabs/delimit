use super::*;

mod convert;
mod variant;

#[derive(Clone, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub enum Apex {
    Void(Hub<()>),
    String(Hub<String>),
    U8(Hub<u8>),
    I32(Hub<i32>),
    Vu8(Hub<Vec<u8>>),
    Vu16(Hub<Vec<u16>>),
    Vf32(Hub<Vf32>),
    Vf64(Hub<Vf64>),
}

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
    pub async fn poll(&self) -> Result<()> {
        match self {
            Self::Void(x) => x.poll().await,
            Self::String(x) => x.poll().await,
            Self::U8(x) => x.poll().await,
            Self::I32(x) => x.poll().await,
            Self::Vu8(x) => x.poll().await,
            Self::Vu16(x) => x.poll().await,
            Self::Vf32(x) => x.poll().await,
            Self::Vf64(x) => x.poll().await,
        }
    }
    pub fn get(&self, aim: impl Into<Aim>) -> Result<Apex> {
        match self {
            Self::Void(x) => x.get(aim),
            Self::String(x) => x.get(aim),
            Self::U8(x) => x.get(aim),
            Self::I32(x) => x.get(aim),
            Self::Vu8(x) => x.get(aim),
            Self::Vu16(x) => x.get(aim),
            Self::Vf32(x) => x.get(aim),
            Self::Vf64(x) => x.get(aim),
        }
    }
    pub async fn set(&self, aim: impl Into<Aim>, apex: impl Into<Apex>) -> Result<()> {
        match self {
            Self::Void(x) => x.set(aim, apex).await,
            Self::String(x) => x.set(aim, apex).await,
            Self::U8(x) => x.set(aim, apex).await,
            Self::I32(x) => x.set(aim, apex).await,
            Self::Vu8(x) => x.set(aim, apex).await,
            Self::Vu16(x) => x.set(aim, apex).await,
            Self::Vf32(x) => x.set(aim, apex).await,
            Self::Vf64(x) => x.set(aim, apex).await,
        }
    }
    pub fn adapt_get(&self, deal: &mut dyn Deal) -> Result<()> {
        match self {
            Self::Void(x) => x.adapt_get(deal),
            Self::String(x) => x.adapt_get(deal),
            Self::U8(x) => x.adapt_get(deal),
            Self::I32(x) => x.adapt_get(deal),
            Self::Vu8(x) => x.adapt_get(deal),
            Self::Vu16(x) => x.adapt_get(deal),
            Self::Vf32(x) => x.adapt_get(deal),
            Self::Vf64(x) => x.adapt_get(deal),
        }
    }
    pub fn transient_set(&self, deal: &mut dyn Deal) -> Result<Ring> {
        match self {
            Self::Void(x) => x.transient_set(deal),
            Self::String(x) => x.transient_set(deal),
            Self::U8(x) => x.transient_set(deal),
            Self::I32(x) => x.transient_set(deal),
            Self::Vu8(x) => x.transient_set(deal),
            Self::Vu16(x) => x.transient_set(deal),
            Self::Vf32(x) => x.transient_set(deal),
            Self::Vf64(x) => x.transient_set(deal),
        }
    }
    pub fn tray_path(&self) -> Option<&Path> {
        match self {
            Self::Void(x) => x.path(),
            Self::String(x) => x.path(),
            Self::U8(x) => x.path(),
            Self::I32(x) => x.path(),
            Self::Vu8(x) => x.path(),
            Self::Vu16(x) => x.path(),
            Self::Vf32(x) => x.path(),
            Self::Vf64(x) => x.path(),
        }
    }
    pub fn pathed(&self, path: impl Into<Path>) -> Self {
        match self {
            Self::Void(x) => Self::Void(x.pathed(path)),
            Self::String(x) => Self::String(x.pathed(path)),
            Self::U8(x) => Self::U8(x.pathed(path)),
            Self::I32(x) => Self::I32(x.pathed(path)),
            Self::Vu8(x) => Self::Vu8(x.pathed(path)),
            Self::Vu16(x) => Self::Vu16(x.pathed(path)),
            Self::Vf32(x) => Self::Vf32(x.pathed(path)),
            Self::Vf64(x) => Self::Vf64(x.pathed(path)),
        }
    }
    pub fn imports(&self) -> Result<Vec<Import>> {
        match self {
            Self::Void(x) => x.imports(),
            Self::String(x) => x.imports(),
            Self::U8(x) => x.imports(),
            Self::I32(x) => x.imports(),
            Self::Vu8(x) => x.imports(),
            Self::Vu16(x) => x.imports(),
            Self::Vf32(x) => x.imports(),
            Self::Vf64(x) => x.imports(),
        }
    }
    pub fn all(&self) -> Result<Vec<Apex>> {
        match self {
            Self::Void(x) => x.all(),
            Self::String(x) => x.all(),
            Self::U8(x) => x.all(),
            Self::I32(x) => x.all(),
            Self::Vu8(x) => x.all(),
            Self::Vu16(x) => x.all(),
            Self::Vf32(x) => x.all(),
            Self::Vf64(x) => x.all(),
        }
    }
    pub fn insert_in_lake(&self, lake: &mut Lake) -> Result<()> {
        match self {
            Self::Void(x) => lake.insert_stem(x),
            Self::String(x) => lake.insert_stem(x),
            Self::U8(x) => lake.insert_stem(x),
            Self::I32(x) => lake.insert_stem(x),
            Self::Vu8(x) => lake.insert_stem(x),
            Self::Vu16(x) => lake.insert_stem(x),
            Self::Vf32(x) => lake.insert_stem(x),
            Self::Vf64(x) => lake.insert_stem(x),
        }
    }
    pub fn grow_from_lake(&self, lake: &mut Lake, ring: &mut Ring) -> Result<()> {
        match self {
            Self::Void(x) => lake.grow(x, ring),
            Self::String(x) => lake.grow(x, ring),
            Self::U8(x) => lake.grow(x, ring),
            Self::I32(x) => lake.grow(x, ring),
            Self::Vu8(x) => lake.grow(x, ring),
            Self::Vu16(x) => lake.grow(x, ring),
            Self::Vf32(x) => lake.grow(x, ring),
            Self::Vf64(x) => lake.grow(x, ring),
        }
    }
}

impl Backed for Apex {
    fn backed(&self, back: &Back) -> Result<Self> {
        let apex = match self {
            Self::Void(x) => Self::Void(x.backed(back)?),
            Self::String(x) => Self::String(x.backed(back)?),
            Self::U8(x) => Self::U8(x.backed(back)?),
            Self::I32(x) => Self::I32(x.backed(back)?),
            Self::Vu8(x) => Self::Vu8(x.backed(back)?),
            Self::Vu16(x) => Self::Vu16(x.backed(back)?),
            Self::Vf32(x) => Self::Vf32(x.backed(back)?),
            Self::Vf64(x) => Self::Vf64(x.backed(back)?),
        };
        Ok(apex)
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
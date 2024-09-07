use super::*;

mod convert;
mod variant;

#[derive(Clone, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub enum Apex {
    Void(Hub<()>),
    String(Hub<String>),
    U8(Hub<u8>),
}

impl Default for Apex {
    fn default() -> Self {
        Self::String("".leaf().hub())
    }
}

impl Apex {
    pub fn get(&self, aim: impl Into<Aim>) -> Result<Apex> {
        match self {
            Self::Void(x) => x.get(aim),
            Self::String(x) => x.get(aim),
            Self::U8(x) => x.get(aim),
        }
    }
    pub fn set(&self, aim: impl Into<Aim>, apex: impl Into<Apex>) -> Result<()> {
        match self {
            Self::Void(x) => x.set(aim, apex),
            Self::String(x) => x.set(aim, apex),
            Self::U8(x) => x.set(aim, apex)
        }
    }
    pub fn hydrate(&self) -> Result<()> {
        let space = Space::new(vec![], self);
        self.saturate(&space, &space)?;
        Ok(())
    }
    fn saturate(&self, world: &Space, local: &Space) -> Result<()> {
        self.adapt(&mut Scope {
            world,
            local,
            back: None,
        })?;
        for space in local.map.values() {
            space.apex.saturate(world, space)?;
        }
        for space in &local.vec {
            space.apex.saturate(world, space)?;
        }
        Ok(())
    }
    pub fn adapt(&self, deal: &mut dyn Deal) -> Result<()> {
        match self {
            Self::Void(x) => x.adapt(deal),
            Self::String(x) => x.adapt(deal),
            Self::U8(x) => x.adapt(deal),
        }
    }
    pub fn path(&self) -> Option<&Path> {
        match self {
            Self::Void(x) => x.path(),
            Self::String(x) => x.path(),
            Self::U8(x) => x.path(),
        }
    }
    pub fn tray_path(&self) -> Option<&Path> {
        match self {
            Self::Void(x) => x.tray_path(),
            Self::String(x) => x.tray_path(),
            Self::U8(x) => x.tray_path(),
        }
    }
    pub fn pathed(&self, path: impl Into<Path>) -> Self {
        match self {
            Self::Void(x) => Self::Void(x.pathed(path)),
            Self::String(x) => Self::String(x.pathed(path)),
            Self::U8(x) => Self::U8(x.pathed(path)),
        }
    }
    pub fn imports(&self) -> Result<Vec<Import>> {
        match self {
            Self::Void(x) => x.imports(),
            Self::String(x) => x.imports(),
            Self::U8(x) => x.imports(),
        }
    }
    pub fn all(&self) -> Result<Vec<Apex>> {
        match self {
            Self::Void(x) => x.all(),
            Self::String(x) => x.all(),
            Self::U8(x) => x.all(),
        }
    }
    pub fn insert_in_lake(&self, lake: &mut Lake) -> Result<()> {
        match self {
            Self::Void(x) => lake.insert_stem(x),
            Self::String(x) => lake.insert_stem(x),
            Self::U8(x) => lake.insert_stem(x),
        }
    }
    pub fn grow_from_lake(&self, lake: &mut Lake) -> Result<()> {
        match self {
            Self::Void(x) => lake.grow(x),
            Self::String(x) => lake.grow(x),
            Self::U8(x) => lake.grow(x),
        }
    }
}

impl TryBacked for Apex {
    type NewSelf = Self;
    fn backed(&self, back: &Back) -> Result<Self::NewSelf> {
        let apex = match self {
            Self::Void(x) => Self::Void(x.backed(back)?),
            Self::String(x) => Self::String(x.backed(back)?),
            Self::U8(x) => Self::U8(x.backed(back)?),
        };
        Ok(apex)
    }
}



// pub trait ToApex {
//     fn apex(&self) -> Apex;
// }

// impl<T: ToApex> ToApex for Vec<T> {
//     fn apex(&self) -> Apex {
//         self.iter().map(|x| x.apex()).collect()
//     }
// }


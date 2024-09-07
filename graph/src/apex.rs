use super::*;

mod convert;

#[derive(Clone, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub enum Apex {
    String(Hub<String>),
    U8(Hub<u8>),
}

impl Default for Apex {
    fn default() -> Self {
        Self::String("".leaf().hub())
    }
}

impl Apex {
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
            Self::String(x) => x.adapt(deal),
            Self::U8(x) => x.adapt(deal),
        }
    }
    pub fn path(&self) -> Option<&Path> {
        match self {
            Self::String(x) => x.path(),
            Self::U8(x) => x.path(),
        }
    }
    pub fn tray_path(&self) -> Option<&Path> {
        match self {
            Self::String(x) => x.tray_path(),
            Self::U8(x) => x.tray_path(),
        }
    }
    pub fn pathed(&self, path: impl Into<Path>) -> Self {
        match self {
            Self::String(x) => Self::String(x.pathed(path)),
            Self::U8(x) => Self::U8(x.pathed(path)),
        }
    }
    pub fn imports(&self) -> Result<Vec<Import>> {
        match self {
            Self::String(x) => x.imports(),
            Self::U8(x) => x.imports(),
        }
    }
    pub fn all(&self) -> Result<Vec<Apex>> {
        match self {
            Self::String(x) => x.all(),
            Self::U8(x) => x.all(),
        }
    }
    pub fn insert_in_lake(&self, lake: &mut Lake) -> Result<()> {
        match self {
            Self::String(x) => lake.insert_stem(x),
            Self::U8(x) => lake.insert_stem(x),
        }
    }
    pub fn grow_from_lake(&self, lake: &mut Lake) -> Result<()> {
        match self {
            Self::String(x) => lake.grow(x),
            Self::U8(x) => lake.grow(x),
        }
    }
}

impl TryBacked for Apex {
    type NewSelf = Self;
    fn backed(&self, back: &Back) -> Result<Self::NewSelf> {
        let apex = match self {
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


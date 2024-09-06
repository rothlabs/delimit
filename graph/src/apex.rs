use super::*;

#[derive(Clone, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub enum Apex {
    String(Hub<String>),
    U8(Hub<u8>),
}

impl Apex {
    pub fn pathed(&self, path: impl Into<Path>) -> Self {
        match self {
            Self::String(x) => Self::String(x.pathed(path)),
            Self::U8(x) => Self::U8(x.pathed(path)),
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

pub enum View<'a> {
    String(&'a mut Hub<String>),
    U8(&'a mut Hub<u8>),
}

impl View<'_> {
    pub fn apex(&self) -> Apex {
        match self {
            Self::String(x) => Apex::String((*x).clone()),
            Self::U8(x) => Apex::U8((*x).clone()),
        }
    }
    pub fn set(self, apex: Apex) -> Result<()> {
        match self {
            Self::String(x) => if let Apex::String(y) = apex {*x = y;},
            Self::U8(x) => if let Apex::U8(y) = apex {*x = y;},
        };
        Ok(())
    }
    pub fn tray_hash(&self) -> Option<u64> {
        match self {
            Self::String(x) => x.tray_hash(),
            Self::U8(x) => x.tray_hash(),
        }
    }
}

impl<'a> From<&'a mut Apex> for View<'a> {
    fn from(value: &'a mut Apex) -> Self {
        match value {
            Apex::String(x) => View::String(x),
            Apex::U8(x) => View::U8(x),
        }
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


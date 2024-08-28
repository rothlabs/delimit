use super::*;
use query::*;
use thiserror::Error;

mod convert;
mod edit;
mod hydrate;
mod import;
mod query;

#[derive(Error, Debug)]
pub enum Error {
    #[error("not ploy")]
    NotPloy,
    #[error("not node")]
    NotNode,
}

// fn not_ploy() -> Result<(), {

// }

/// Primary graph part.
#[derive(Clone, PartialEq, Hash, Serialize, Debug)]
#[serde(untagged)]
pub enum Apex {
    Tray(Tray),
    Leaf(Leaf),
    Ploy(Ploy),
}

impl Apex {
    pub fn none() -> Self {
        Self::default()
    }

    /// Run main apex function. Will return lower rank apex if successful.
    pub fn main(&self) -> Result<Apex, crate::AnyError> {
        match self {
            Self::Ploy(ploy) => ploy.main(),
            _ => Err(Error::NotPloy)?,
        }
    }

    /// Get stem by key
    pub fn get<'a>(&self, query: impl Into<Query<'a>>) -> Result<Apex, solve::Error> {
        match self {
            Self::Ploy(ploy) => match query.into() {
                Query::Key(key) => ploy.solve(Task::Get(&key))?.apex(),
                Query::Keys(keys) => {
                    let apex = ploy.solve(Task::Get(&keys[0]))?.apex();
                    if keys.len() > 1 {
                        apex?.get(&keys[1..])
                    } else {
                        apex
                    }
                }
                Query::Index(index) => {
                    if let Some(apex) = ploy.solve(Task::All)?.apexes()?.get(index) {
                        Ok(apex.clone())
                    } else {
                        Err(solve::Error::IndexOutOfBounds(index))
                    }
                }
            },
            _ => Err(Error::NotPloy)?,
        }
    }

    pub fn imports(&self) -> Result<Vec<Import>, solve::Error> {
        match self {
            Self::Ploy(ploy) => ploy.solve(Task::Imports)?.imports(),
            _ => Err(Error::NotPloy)?,
        }
    }

    // pub fn map(&self) -> Result<Map, solve::Error> {
    //     match self {
    //         Self::Ploy(ploy) => ploy.solve(Task::Map)?.map(),
    //         _ => Err(Error::NotPloy)?,
    //     }
    // }

    /// Insert apex into new Lake.
    pub fn lake(&self) -> Result<Lake, crate::Error> {
        let mut lake = Lake::new();
        lake.insert("root", self)?;
        Ok(lake)
    }

    /// Get hash digest number of apex.
    pub fn digest(&self) -> Result<u64, solve::Error> {
        match self {
            Self::Leaf(leaf) => leaf.solve(Task::Hash),
            Self::Ploy(ploy) => ploy.solve(Task::Hash),
            _ => Err(Error::NotNode)?,
        }?
        .u64()
    }

    /// Get serial string of apex.
    pub fn serial(&self) -> Result<String, solve::Error> {
        match self {
            Self::Tray(tray) => tray.serial(),
            Self::Leaf(leaf) => leaf.solve(Task::Serial),
            Self::Ploy(ploy) => ploy.solve(Task::Serial),
        }?
        .string()
    }

    /// Get stems of apex.
    pub fn stems(&self) -> Result<Vec<Apex>, solve::Error> {
        match self {
            Self::Ploy(ploy) => ploy.solve(Task::All),
            _ => Err(Error::NotPloy)?,
        }?
        .apexes()
    }

    /// Replace stems according to the Trade deal.
    pub fn trade(&self, deal: &dyn Trade) {
        if let Self::Ploy(ploy) = self {
            ploy.adapt(Post::Trade(deal)).ok();
        }
    }

    /// New Apex with Path
    pub fn pathed(&self, path: impl Into<Path>) -> Self {
        match self {
            Self::Tray(bare) => Self::Tray(bare.clone()),
            Self::Leaf(leaf) => Self::Leaf(leaf.pathed(path.into())),
            Self::Ploy(ploy) => Self::Ploy(ploy.pathed(path.into())),
        }
    }

    /// Get path associated with apex if any.
    pub fn path(&self) -> Option<&Path> {
        match self {
            Self::Tray(tray) => tray.path(),
            Self::Leaf(leaf) => leaf.path(),
            Self::Ploy(ploy) => ploy.path(),
        }
    }

    /// Get tray of apex. Will solve to lowest rank if needed.
    pub fn tray(&self) -> tray::Result {
        match self {
            Self::Tray(bare) => Ok(bare.clone()),
            Self::Leaf(leaf) => Ok(leaf.tray()),
            Self::Ploy(ploy) => ploy.main()?.tray(),
        }
    }

    /// Run Trade deal with this apex as input.
    pub fn deal(&self, deal: &dyn Trade) -> Self {
        deal.trade(self)
    }

    /// Get rank of apex. Rank 1 apexes produce leaf apexes.
    pub fn rank(&self) -> Option<usize> {
        match self {
            Self::Ploy(ploy) => ploy.rank(),
            _ => None,
        }
    }

    /// Solve down to the given graph rank.
    pub fn at(&self, target: usize) -> Result<Apex, crate::AnyError> {
        let mut apex = self.clone();
        let mut rank = apex.rank();
        while let Some(current) = rank {
            if current > target {
                apex = apex.main()?;
                rank = apex.rank();
            } else {
                rank = None;
            }
        }
        Ok(apex)
    }

    /// New backed apex.
    pub fn backed(&self, back: &Back) -> Self {
        match self {
            Self::Tray(bare) => Self::Tray(bare.clone()),
            Self::Leaf(leaf) => Self::Leaf(leaf.backed(back)),
            Self::Ploy(ploy) => Self::Ploy(ploy.backed(back)),
        }
    }

    /// Read contents of apex.
    pub fn read<T, F: FnOnce(tray::ResultRef) -> T>(&self, read: F) -> T {
        match self {
            Self::Tray(bare) => read(Ok(bare)),
            Self::Leaf(leaf) => leaf.read_tray(read),
            Self::Ploy(ploy) => {
                if let Ok(apex) = ploy.main() {
                    apex.read(read)
                } else {
                    read(Err("failed to read ploy".into()))
                }
            }
        }
    }

    pub fn read_or_error<T, F: FnOnce(&Tray) -> T>(&self, read: F) -> Result<T, crate::AnyError> {
        self.read(|tray| match tray {
            Ok(value) => Ok(read(value)),
            _ => Err("nothing to read")?,
        })
    }
    pub fn read_string<T, F: FnOnce(&String) -> T>(&self, read: F) -> T {
        self.read(|tray| match tray {
            Ok(Tray::String(value)) => read(value),
            _ => read(&"".into()),
        })
    }
    pub fn read_vu8<T, F: FnOnce(&Vec<u8>) -> T>(&self, read: F) -> T {
        self.read(|tray| match tray {
            Ok(Tray::Vu8(value)) => read(value),
            _ => read(&vec![]),
        })
    }
    pub fn read_vu16<T, F: FnOnce(&Vec<u16>) -> T>(&self, read: F) -> T {
        self.read(|tray| match tray {
            Ok(Tray::Vu16(value)) => read(value),
            _ => read(&vec![]),
        })
    }
    pub fn read_vf32<T, F: FnOnce(&Vec<f32>) -> T>(&self, read: F) -> T {
        self.read(|tray| match tray {
            Ok(Tray::Vf32(value)) => read(value),
            _ => read(&vec![]),
        })
    }
    pub fn string(&self) -> Result<String, crate::AnyError> {
        match self.tray() {
            Ok(Tray::String(value)) => Ok(value),
            _ => Err("not a string")?,
        }
    }
    pub fn u32(&self) -> u32 {
        match self.tray() {
            Ok(Tray::U32(value)) => value,
            _ => 0,
        }
    }
    pub fn i32(&self) -> i32 {
        match self.tray() {
            Ok(Tray::I32(value)) => value,
            _ => 0,
        }
    }
}

impl Default for Apex {
    fn default() -> Self {
        Self::Tray(Tray::None)
    }
}

pub trait EngageApexes {
    /// Solve down to the given graph rank.
    fn at(&self, rank: usize) -> Result<Vec<Apex>, crate::AnyError>;
    /// Replace stems according to the Trade deal.
    fn deal(&self, deal: &dyn Trade) -> Self;
}

impl EngageApexes for Vec<Apex> {
    fn at(&self, rank: usize) -> Result<Vec<Apex>, crate::AnyError> {
        self.iter().map(|x| x.at(rank)).collect()
    }
    fn deal(&self, deal: &dyn Trade) -> Self {
        self.iter().map(|x| x.deal(deal)).collect()
    }
}

// Self::Tray(tray) => {
//     let mut state = DefaultHasher::new();
//     tray.hash(&mut state);
//     state.finish().gain()
// },

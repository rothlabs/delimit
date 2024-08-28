use super::*;
use query::*;
use thiserror::Error;
use view::*;

mod convert;
mod edit;
mod hydrate;
mod import;
mod query;
mod view;

#[derive(Error, Debug)]
pub enum Error {
    #[error("not ploy")]
    NotPloy,
    #[error("not ploy or leaf")]
    NotNode,
    #[error("wrong tray (expected: {expected:?}, found: {found:?})")]
    WrongTray{expected: String, found: Tray},
}

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
    pub fn main(&self) -> GraphResult<Apex> {
        match self {
            Self::Ploy(ploy) => ploy.main(),
            _ => Err(Error::NotPloy)?,
        }
    }

    /// Get stem by key
    pub fn get<'a>(&self, query: impl Into<Query<'a>>) -> GraphResult<Apex> {
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
                        Err(solve::Error::IndexOutOfBounds(index))?
                    }
                }
            },
            _ => Err(Error::NotPloy)?,
        }
    }

    pub fn imports(&self) -> GraphResult<Vec<Import>> {
        match self {
            Self::Ploy(ploy) => ploy.solve(Task::Imports)?.imports(),
            _ => Err(Error::NotPloy)?,
        }
    }

    /// Insert apex into new Lake.
    pub fn lake(&self) -> GraphResult<Lake> {
        let mut lake = Lake::new();
        lake.insert("root", self)?;
        Ok(lake)
    }

    /// Get hash digest number of apex.
    pub fn digest(&self) -> GraphResult<u64> {
        match self {
            Self::Leaf(leaf) => leaf.solve(Task::Hash),
            Self::Ploy(ploy) => ploy.solve(Task::Hash),
            _ => Err(Error::NotNode)?,
        }?
        .u64()
    }

    /// Get serial string of apex.
    pub fn serial(&self) -> GraphResult<String> {
        match self {
            Self::Tray(tray) => tray.serial(),
            Self::Leaf(leaf) => leaf.solve(Task::Serial),
            Self::Ploy(ploy) => ploy.solve(Task::Serial),
        }?
        .string()
    }

    /// Get stems of apex.
    pub fn stems(&self) -> GraphResult<Vec<Apex>> {
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
    pub fn tray(&self) -> GraphResult<Tray> {
        match self {
            Self::Tray(bare) => Ok(bare.clone()),
            Self::Leaf(leaf) => leaf.tray(),
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
    pub fn at(&self, target: usize) -> GraphResult<Apex> {
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
            // TODO: remove unwrap!
            Self::Ploy(ploy) => Self::Ploy(ploy.backed(back).unwrap()),
        }
    }

    /// Read tray of apex.
    pub fn read<T, F: FnOnce(GraphResult<&Tray>) -> GraphResult<T>>(&self, read: F) -> GraphResult<T> {
        match self {
            Self::Tray(bare) => read(Ok(bare)),
            Self::Leaf(leaf) => leaf.read(read),
            Self::Ploy(ploy) => ploy.main()?.read(read),
        }
    }

    /// Make a View for reading Tray variants.
    pub fn view(&self) -> View {
        View {apex: self}
    }

    /// Clone of String from Apex.
    pub fn string(&self) -> GraphResult<String> {
        let tray = self.tray()?;
        match tray {
            Tray::String(value) => Ok(value),
            _ => Err(wrong_tray("String", tray))?
        }
    }

    /// u32 from Apex.
    pub fn u32(&self) -> GraphResult<u32> {
        let tray = self.tray()?;
        match tray {
            Tray::U32(value) => Ok(value),
            _ => Err(wrong_tray("u32", tray))?
        }
    }

    /// i32 from Apex.
    pub fn i32(&self) -> GraphResult<i32> {
        let tray = self.tray()?;
        match tray {
            Tray::I32(value) => Ok(value),
            _ => Err(wrong_tray("i32", tray))?
        }
    }
}

pub fn wrong_tray(expected: &str, found: Tray) -> Error {
    Error::WrongTray { expected: expected.into(), found }
}

impl Default for Apex {
    fn default() -> Self {
        Self::Tray(Tray::None)
    }
}

pub trait EngageApexes {
    /// Solve down to the given graph rank.
    fn at(&self, rank: usize) -> Result<Vec<Apex>, crate::Error>;
    /// Replace stems according to the Trade deal.
    fn deal(&self, deal: &dyn Trade) -> Self;
}

impl EngageApexes for Vec<Apex> {
    fn at(&self, rank: usize) -> Result<Vec<Apex>, crate::Error> {
        self.iter().map(|x| x.at(rank)).collect()
    }
    fn deal(&self, deal: &dyn Trade) -> Self {
        self.iter().map(|x| x.deal(deal)).collect()
    }
}
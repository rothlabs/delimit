use super::*;
use thiserror::Error;
use view::*;

mod convert;
mod deserialize;
mod get;
mod hydrate;
mod set;
mod view;

#[derive(Error, Debug)]
pub enum Error {
    #[error("not ploy")]
    NotPloy,
    #[error("not ploy or leaf")]
    NotNode,
    #[error("not found: ({0:?})")]
    NotFound(Vec<aim::Error>),
}

/// Primary graph part.
#[derive(Clone, PartialEq, Hash, Serialize, Debug)]
#[serde(untagged)]
pub enum Hub<T> {
    Tray(T),
    Leaf(Leaf<T>),
    Ploy(Ploy<T>),
}

impl<T> Hub<T> {
    pub fn none() -> Self {
        Self::default()
    }

    /// Run main hub function. Will return lower rank hub if successful.
    pub fn main(&self) -> Result<Hub> {
        match self {
            Self::Ploy(ploy) => ploy.main(),
            _ => Err(Error::NotPloy)?,
        }
    }

    pub fn imports(&self) -> Result<Vec<Import>> {
        match self {
            Self::Ploy(ploy) => ploy.solve(Task::Imports)?.imports(),
            _ => Err(Error::NotPloy)?,
        }
    }

    /// Insert hub into new Lake.
    pub fn lake(&self) -> Result<Lake> {
        let mut lake = Lake::new();
        lake.insert("root", self)?;
        Ok(lake)
    }

    /// Get hash digest number of hub.
    pub fn digest(&self) -> Result<u64> {
        match self {
            Self::Leaf(leaf) => leaf.solve(Task::Hash),
            Self::Ploy(ploy) => ploy.solve(Task::Hash),
            _ => Err(Error::NotNode)?,
        }?
        .u64()
    }

    /// Get serial string of hub.
    pub fn serial(&self) -> Result<String> {
        match self {
            Self::Tray(tray) => tray.serial(),
            Self::Leaf(leaf) => leaf.solve(Task::Serial),
            Self::Ploy(ploy) => ploy.solve(Task::Serial),
        }?
        .string()
    }

    /// Replace stems according to the Trade deal.
    pub fn adapt(&self, deal: &mut dyn Deal) -> Result<()> {
        if let Self::Ploy(ploy) = self {
            ploy.adapt(deal)?;
        }
        Ok(())
    }

    /// New Hub with Path
    pub fn pathed(&self, path: impl Into<Path>) -> Self {
        match self {
            Self::Tray(bare) => Self::Tray(bare.clone()),
            Self::Leaf(leaf) => Self::Leaf(leaf.pathed(path.into())),
            Self::Ploy(ploy) => Self::Ploy(ploy.pathed(path.into())),
        }
    }

    /// Get path associated with hub if any.
    pub fn path(&self) -> Option<&Path> {
        match self {
            Self::Tray(tray) => tray.path(),
            Self::Leaf(leaf) => leaf.path(),
            Self::Ploy(ploy) => ploy.path(),
        }
    }

    /// Get tray of hub. Will solve to lowest rank if needed.
    pub fn tray(&self) -> Result<Tray> {
        match self {
            Self::Tray(tray) => Ok(tray.clone()),
            Self::Leaf(leaf) => leaf.tray(),
            Self::Ploy(ploy) => ploy.main()?.tray(),
        }
    }

    /// Run Trade deal with this hub as input.
    pub fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
        deal.one(key, self)
    }

    /// Get rank of hub. Rank 1 hubes produce leaf hubes.
    pub fn rank(&self) -> Option<u64> {
        match self {
            Self::Ploy(ploy) => ploy.rank(),
            _ => None,
        }
    }

    /// Solve down to the given graph rank.
    pub fn down(&self, target: u64) -> Result<Hub> {
        let mut hub = self.clone();
        let mut rank = hub.rank();
        while let Some(current) = rank {
            if current > target {
                hub = hub.main()?;
                rank = hub.rank();
            } else {
                rank = None;
            }
        }
        Ok(hub)
    }

    /// Read tray of hub.
    pub fn read<T, F: FnOnce(&Tray) -> T>(&self, read: F) -> Result<T> {
        match self {
            Self::Tray(bare) => Ok(read(bare)),
            Self::Leaf(leaf) => leaf.read(read),
            Self::Ploy(ploy) => ploy.main()?.read(read),
        }
    }

    /// Make a View for reading Tray variants.
    pub fn view(&self) -> View {
        View { hub: self }
    }

    /// Clone String from Hub.
    pub fn string(&self) -> Result<String> {
        let tray = self.tray()?;
        match tray {
            Tray::String(value) => Ok(value),
            _ => Err(tray.wrong_variant("String"))?,
        }
    }

    /// u32 from Hub.
    pub fn u32(&self) -> Result<u32> {
        let tray = self.tray()?;
        match tray {
            Tray::U32(value) => Ok(value),
            _ => Err(tray.wrong_variant("u32"))?,
        }
    }

    /// i32 from Hub.
    pub fn i32(&self) -> Result<i32> {
        let tray = self.tray()?;
        match tray {
            Tray::I32(value) => Ok(value),
            _ => Err(tray.wrong_variant("i32"))?,
        }
    }
}

impl TryBacked for Hub {
    type Out = Self;
    /// New backed hub.
    fn backed(&self, back: &Back) -> Result<Self> {
        match self {
            Self::Tray(bare) => Ok(Self::Tray(bare.clone())),
            Self::Leaf(leaf) => Ok(Self::Leaf(leaf.backed(back)?)),
            Self::Ploy(ploy) => Ok(Self::Ploy(ploy.backed(back)?)),
        }
    }
}

impl Default for Hub {
    fn default() -> Self {
        Self::Tray(Tray::None)
    }
}

pub trait EngageHubes<'a> {
    /// Solve down to the given graph rank.
    fn down(&self, rank: u64) -> Result<Vec<Hub>>;
    /// Replace stems according to the Trade deal.
    fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()>;
}

impl<'a> EngageHubes<'a> for Vec<Hub> {
    fn down(&self, rank: u64) -> Result<Vec<Hub>> {
        self.iter().map(|x| x.down(rank)).collect()
    }
    fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
        deal.vec(key, self)
    }
}

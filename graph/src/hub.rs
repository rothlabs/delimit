use super::*;
use thiserror::Error;

mod convert;
mod deserialize;
mod get;
mod hydrate;
mod set;

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
// #[serde(untagged)]
pub enum Hub<T> 
where 
    T: 'static + Payload
{
    Tray(Tray<T>),
    Leaf(Leaf<T>),
    Ploy(Ploy<T>),
}

impl<T> Hub<T> 
where 
    T: Payload
{
    pub fn none() -> Self {
        Self::default()
    }

    /// Run main hub function. Will return lower rank hub if successful.
    pub fn main(&self) -> Result<Hub<T>> {
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

    pub fn tray_path(&self) -> Option<&Path> {
        if let Hub::Tray(Tray::Path(path)) = self {
            Some(path)
        } else {
            None
        }
    }

    pub fn tray_hash(&self) -> Option<u64> {
        if let Hub::Tray(Tray::Path(Path::Hash(hash))) = self {
            Some(*hash)
        } else {
            None
        }
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
            Self::Tray(tray) => Self::Tray(tray.clone()),
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
    pub fn tray(&self) -> Result<Tray<T>> {
        match self {
            Self::Tray(tray) => Ok(tray.clone()),
            Self::Leaf(leaf) => leaf.tray(),
            Self::Ploy(ploy) => ploy.main()?.tray(),
        }
    }

    // /// Run Trade deal with this hub as input.
    // pub fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
    //     deal.one(key, self)
    // }

    /// Get rank of hub. Rank 1 hubes produce leaf hubes.
    pub fn rank(&self) -> Option<u64> {
        match self {
            Self::Ploy(ploy) => ploy.rank(),
            _ => None,
        }
    }

    /// Solve down to the given graph rank.
    pub fn down(&self, target: u64) -> Result<Hub<T>> {
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
    pub fn read<O, F: FnOnce(&T) -> O>(&self, read: F) -> Result<O> {
        match self {
            Self::Tray(tray) => {
                if let Tray::Item(item) = tray {
                    Ok(read(item))
                } else {
                    Err(tray.wrong_variant("Item"))?
                }
            },
            Self::Leaf(leaf) => leaf.read(read),
            Self::Ploy(ploy) => ploy.main()?.read(read),
        }
    }

    // /// Make a View for reading Tray variants.
    // pub fn view(&self) -> View {
    //     View { hub: self }
    // }

    /// Clone String from Hub.
    pub fn item(&self) -> Result<T> {
        let tray = self.tray()?;
        match tray {
            Tray::Item(item) => Ok(item),
            _ => Err(tray.wrong_variant("Item"))?,
        }
    }
}

impl<T> TryBacked for Hub<T> 
where 
    T: Payload
{
    type NewSelf = Self;
    /// New backed hub.
    fn backed(&self, back: &Back) -> Result<Self> {
        match self {
            Self::Tray(tray) => Ok(Self::Tray(tray.clone())),
            Self::Leaf(leaf) => Ok(Self::Leaf(leaf.backed(back)?)),
            Self::Ploy(ploy) => Ok(Self::Ploy(ploy.backed(back)?)),
        }
    }
}

impl<T> Default for Hub<T> 
where 
    T: Payload
{
    fn default() -> Self {
        Self::Tray(Tray::None)
    }
}

pub trait EngageHubes<'a, T> 
where 
    T: Payload
{
    /// Solve down to the given graph rank.
    fn down(&self, rank: u64) -> Result<Vec<Hub<T>>>;
    // Replace stems according to the Trade deal.
    // fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()>;
}

impl<'a, T> EngageHubes<'a, T> for Vec<Hub<T>> 
where 
    T: Payload
{
    fn down(&self, rank: u64) -> Result<Vec<Hub<T>>> {
        self.iter().map(|x| x.down(rank)).collect()
    }
    // fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
    //     deal.vec(key, self)
    // }
}

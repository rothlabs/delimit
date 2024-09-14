use std::{future::Future, task::Poll};

pub use deal::*;

use super::*;
use thiserror::Error;

mod convert;
mod deal;
mod deserialize;
mod get;
mod set;

#[derive(Error, Debug)]
pub enum Error {
    #[error("not ploy")]
    NotPloy,
    #[error("not ploy or leaf")]
    NotNode,
    #[error("not found: ({0:?})")]
    NotFound(Vec<crate::Error>),
}

/// Primary graph part.
#[derive(Clone, PartialEq, Hash, Serialize, Debug)]
#[serde(untagged)]
pub enum Hub<T>
where
    T: 'static + Payload,
{
    Tray(Tray<T>),
    Leaf(Leaf<T>),
    Ploy(Ploy<T>),
}

impl<T> Hub<T>
where
    T: Payload,
{
    pub fn none() -> Self {
        Self::default()
    }

    /// Run main hub function. Will return lower rank hub if successful.
    pub async fn main(&self) -> Result<Hub<T>> {
        match self {
            Self::Ploy(ploy) => ploy.main().await,
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

    /// Literal `Path` if present. This does not return the `Link` path.
    pub fn path(&self) -> Option<&Path> {
        if let Hub::Tray(Tray::Path(path)) = self {
            Some(path)
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

    /// Get rank of hub. Rank 1 hubes produce leaf hubes.
    pub fn rank(&self) -> Option<u64> {
        match self {
            Self::Ploy(ploy) => ploy.rank(),
            _ => None,
        }
    }

    /// Solve down to the given graph rank.
    pub async fn down(&self, target: u64) -> Result<Hub<T>> {
        let mut hub = self.clone();
        let mut rank = hub.rank();
        while let Some(current) = rank {
            if current > target {
                hub = hub.main().await?;
                rank = hub.rank();
            } else {
                rank = None;
            }
        }
        Ok(hub)
    }

    /// Read tray of hub.
    pub async fn read<O, F: 'static + FnOnce(&T) -> O>(&self, read: F) -> Result<O> { // Future<Output = 
        match self {
            Self::Tray(tray) => {
                if let Tray::Base(base) = tray {
                    Ok(read(base))
                } else {
                    Err(tray.wrong_variant("Base"))?
                }
            }
            Self::Leaf(leaf) => leaf.read(read),
            Self::Ploy(ploy) => ploy.main().await?.read(read).await,
        }
    }

    /// Base value. The graph is solved down to the base.
    pub async fn base(&self) -> Result<T> {
        match self {
            Self::Tray(tray) => match tray {
                Tray::Base(base) => Ok(base.clone()),
                tray => Err(tray.wrong_variant("Base"))?,
            },
            Self::Leaf(leaf) => leaf.read(|base| base.clone()),
            Self::Ploy(ploy) => ploy.main().await?.base().await,
        }
    }
}

impl<T> Backed for Hub<T>
where
    T: Payload,
{
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
    T: Payload,
{
    fn default() -> Self {
        Self::Tray(Tray::None)
    }
}

pub trait SolveDown<'a, T>
where
    T: 'static + Payload,
{
    /// Solve down to the given graph rank.
    async fn down(&self, rank: u64) -> Result<Vec<Hub<T>>>;
}

impl<'a, T> SolveDown<'a, T> for Vec<Hub<T>>
where
    T: 'static + Payload,
{
    async fn down(&self, rank: u64) -> Result<Vec<Hub<T>>> {
        let mut out = vec![];
        for hub in self {
            out.push(hub.down(rank).await?);
        }
        Ok(out)
        // self.iter().map(|x| x.down(rank)).collect() // join_all();
    }
}

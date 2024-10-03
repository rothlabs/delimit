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
#[derive(Clone, PartialEq, Serialize, Debug)]
#[serde(untagged)]
pub enum Hub<T>
where
    T: 'static + Payload,
{
    Tray(Tray<T>),
    Leaf(Leaf<T>),
    Ploy(Ploy<T>),
}

impl<T: 'static + Payload + HashGraph> HashGraph for Hub<T> {
    fn hash_graph<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Tray(x) => x.hash_graph(state),
            Self::Leaf(x) => x.hash_graph(state),
            Self::Ploy(x) => x.hash_graph(state),
        }
    }
}

impl<T: Payload> Hub<T> {
    pub fn none() -> Self {
        Self::default()
    }

    /// Run main hub function. Will return lower rank hub if successful.
    pub async fn main(&self) -> Result<Hub<T>> {
        match self {
            Self::Ploy(ploy) => ploy.solve().await,
            _ => Err(Error::NotPloy)?,
        }
    }

    pub fn imports(&self) -> Result<Vec<Import>> {
        match self {
            Self::Ploy(ploy) => ploy.reckon(Task::Imports)?.imports(),
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
            Self::Leaf(leaf) => leaf.reckon(Task::Hash),
            Self::Ploy(ploy) => ploy.reckon(Task::Hash),
            _ => Err(Error::NotNode)?,
        }?
        .u64()
    }

    /// Get serial string of hub.
    pub fn serial(&self) -> Result<String> {
        match self {
            Self::Tray(tray) => tray.serial(),
            Self::Leaf(leaf) => leaf.reckon(Task::Serial),
            Self::Ploy(ploy) => ploy.reckon(Task::Serial),
        }?
        .string()
    }

    /// Replace stems according to the Trade deal.
    pub fn adapt_get(&self, deal: &mut dyn Deal) -> Result<()> {
        if let Self::Ploy(ploy) = self {
            ploy.adapt_get(deal)?;
        }
        Ok(())
    }

    /// Replace stems according to the Trade deal.
    pub async fn adapt_set(&self, deal: &mut dyn Deal) -> Result<()> {
        if let Self::Ploy(ploy) = self {
            ploy.adapt_set(deal).await?;
        }
        Ok(())
    }

    pub fn transient_set(&self, deal: &mut dyn Deal) -> Result<Ring> {
        if let Self::Ploy(ploy) = self {
            ploy.transient_set(deal)
        } else {
            Ok(Ring::new())
        }
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
    pub fn read<'a, O, F>(&'a self, read: F) -> GraphFuture<'a, Result<O>>
    where
        F: FnOnce(&T) -> O + 'a + IsSend,
    {
        Box::pin(async move {
            match self {
                Self::Tray(tray) => {
                    if let Tray::Base(base) = tray {
                        Ok(read(base))
                    } else {
                        Err(tray.wrong_variant("Base"))?
                    }
                }
                Self::Leaf(leaf) => leaf.read(read),
                Self::Ploy(ploy) => ploy.solve().await?.read(read).await,
            }
        })
    }

    pub fn base(&self) -> GraphFuture<Result<T>> {
        Box::pin(async move {
            match self {
                Self::Tray(tray) => match tray {
                    Tray::Base(base) => Ok(base.clone()),
                    tray => Err(tray.wrong_variant("Base"))?,
                },
                Self::Leaf(leaf) => leaf.read(|base| base.clone()),
                Self::Ploy(ploy) => ploy.solve().await?.base().await,
            }
        })
    }

    pub async fn poll(&self) -> Result<()> {
        self.base().await?;
        Ok(())
    }
}

impl<T: Payload> Backed for Hub<T> {
    fn backed(&self, back: &Back) -> Result<Self> {
        match self {
            Self::Tray(tray) => Ok(Self::Tray(tray.clone())),
            Self::Leaf(leaf) => Ok(Self::Leaf(leaf.backed(back)?)),
            Self::Ploy(ploy) => Ok(Self::Ploy(ploy.backed(back)?)),
        }
    }
}

impl<T: Payload> Default for Hub<T> {
    fn default() -> Self {
        Self::Tray(Tray::Base(T::default()))
    }
}

pub trait SolveDown<T>
where
    T: 'static + Payload,
{
    /// Solve down to the given graph rank.
    fn down(&self, rank: u64) -> impl Future<Output = Result<Vec<Hub<T>>>> + IsSend;
}

impl<T: Payload> SolveDown<T> for Vec<Hub<T>> {
    async fn down(&self, rank: u64) -> Result<Vec<Hub<T>>> {
        let mut out = vec![];
        for hub in self {
            out.push(hub.down(rank).await?);
        }
        Ok(out)
    }
}

impl<T: Payload> HashGraph for Vec<Hub<T>> {
    fn hash_graph<H: Hasher>(&self, state: &mut H) {
        for hub in self {
            hub.hash_graph(state);
        }
    }
}

impl<T: Payload> HashGraph for Option<T> {
    fn hash_graph<H: Hasher>(&self, state: &mut H) {
        if let Some(x) = self {
            x.hash_graph(state);
        }
    }
}

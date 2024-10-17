pub use convert::{ToPloyHub, ToWingHub};

use super::*;
use thiserror::Error;

mod convert;
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
#[derive(Clone, PartialEq, Debug)] // Serialize
// #[serde(untagged)]
pub enum Hub<T> { // : Payload
    /// A base value or Path
    Tray(Tray<T>),
    /// Graph leaf node
    Leaf(Leaf<T>),
    /// Node with type-erased unit. Can be serialized.
    Ploy(Ploy<T>),
    /// Node with type-erased unit. Cannot be serialized.
    Wing(Wing<T>),
}

impl<T> Serialize for Hub<T> 
where 
    T: Digest + Serialize
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        match self {
            Self::Tray(x) => x.serialize(serializer),
            Self::Leaf(x) => x.serialize(serializer),
            Self::Ploy(x) => x.serialize(serializer),
            Self::Wing(_) => serializer.serialize_unit(),
        }
    }
}

impl<T> Digest for Hub<T>
where
    T: Digest + Serialize,
{
    fn digest<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Tray(x) => x.digest(state),
            Self::Leaf(x) => x.digest(state),
            Self::Ploy(x) => x.digest(state),
            Self::Wing(_) => (),
        }
    }
}

impl<T> Hub<T> {
    pub fn none() -> Self {
        Self::default()
    }

    /// Get rank of hub. Rank 1 hubes produce leaf hubes.
    pub fn rank(&self) -> Option<u16> {
        match self {
            Self::Ploy(ploy) => ploy.rank(),
            Self::Wing(wing) => wing.rank(),
            _ => None,
        }
    }
}

impl<T> Hub<T> 
where 
    T: SendSync + Debug + Clone,
{
    /// Run main hub function. Will return lower rank hub if successful.
    pub async fn main(&self) -> Result<Hub<T>> {
        match self {
            Self::Ploy(ploy) => ploy.solve().await,
            // Self::Wing(wing) => wing.solve().await,
            _ => Err(Error::NotPloy)?,
        }
    }

    /// Solve down to the given graph rank (level).
    pub async fn down(&self, target: u16) -> Result<Hub<T>> {
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

    // Could use a loop instead of recursion and remove the Box::pin(async move {
    // see Hub::down
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
                Self::Wing(wing) => wing.solve().await?.read(read).await,
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
                Self::Wing(wing) => wing.solve().await?.base().await,
            }
        })
    }

    pub fn depend(&self) -> GraphFuture<Result<()>> {
        Box::pin(async move {
            match self {
                Self::Tray(_) => Ok(()),
                Self::Leaf(leaf) => leaf.read(|_| ()),
                Self::Ploy(ploy) => ploy.solve().await?.depend().await,
                Self::Wing(wing) => wing.solve().await?.depend().await,
            }
        })
    }
}

impl<T: Payload> Hub<T> {
    // pub fn none() -> Self {
    //     Self::default()
    // }

    // pub async fn main(&self) -> Result<Hub<T>> {
    //     match self {
    //         Self::Ploy(ploy) => ploy.solve().await,
    //         // Self::Wing(wing) => wing.solve().await,
    //         _ => Err(Error::NotPloy)?,
    //     }
    // }

    pub fn imports(&self) -> Result<Vec<Import>> {
        match self {
            Self::Ploy(ploy) => ploy.get_imports(),
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
    pub fn get_hash(&self) -> Result<u64> {
        match self {
            Self::Leaf(leaf) => leaf.get_hash(),
            Self::Ploy(ploy) => ploy.get_hash(),
            Self::Wing(_) => Ok(0),
            _ => Err(Error::NotNode)?,
        }
    }

    /// Get serial string of hub.
    pub fn serial(&self) -> Result<String> {
        match self {
            Self::Tray(tray) => tray.serial(),
            Self::Leaf(leaf) => leaf.get_serial(),
            Self::Ploy(ploy) => ploy.get_serial(),
            Self::Wing(wing) => wing.serial(),
        }
    }

    /// Replace stems according to the Trade deal.
    pub fn adapt_get(&self, deal: &mut dyn Deal) -> Result<()> {
        match self {
            Self::Ploy(ploy) => ploy.adapt_get(deal),
            Self::Wing(wing) => wing.adapt_get(deal),
            _ => Ok(()),
        }
    }

    /// Replace stems according to the Trade deal.
    pub async fn adapt_set(&self, deal: &mut dyn Deal) -> Result<()> {
        match self {
            Self::Ploy(ploy) => ploy.adapt_set(deal).await,
            Self::Wing(wing) => wing.adapt_set(deal).await,
            _ => Ok(()),
        }
    }

    pub fn transient_set(&self, deal: &mut dyn Deal) -> Result<Ring> {
        match self {
            Self::Ploy(ploy) => ploy.transient_set(deal),
            Self::Wing(wing) => wing.transient_set(deal),
            _ => Ok(Ring::new()),
        }
    }

    /// New Hub with Path
    pub fn pathed(&self, path: impl Into<Path>) -> Self {
        match self {
            Self::Tray(tray) => Self::Tray(tray.clone()),
            Self::Leaf(leaf) => Self::Leaf(leaf.pathed(path.into())),
            Self::Ploy(ploy) => Self::Ploy(ploy.pathed(path.into())),
            Self::Wing(wing) => Self::Wing(wing.pathed(path.into())),
        }
    }

    // /// Get rank of hub. Rank 1 hubes produce leaf hubes.
    // pub fn rank(&self) -> Option<u16> {
    //     match self {
    //         Self::Ploy(ploy) => ploy.rank(),
    //         Self::Wing(wing) => wing.rank(),
    //         _ => None,
    //     }
    // }

    // /// Solve down to the given graph rank (level).
    // pub async fn down(&self, target: u16) -> Result<Hub<T>> {
    //     let mut hub = self.clone();
    //     let mut rank = hub.rank();
    //     while let Some(current) = rank {
    //         if current > target {
    //             hub = hub.main().await?;
    //             rank = hub.rank();
    //         } else {
    //             rank = None;
    //         }
    //     }
    //     Ok(hub)
    // }

    // // Could use a loop instead of recursion and remove the Box::pin(async move {
    //     // see Hub::down
    // pub fn read<'a, O, F>(&'a self, read: F) -> GraphFuture<'a, Result<O>>
    // where
    //     F: FnOnce(&T) -> O + 'a + IsSend,
    // {
    //     Box::pin(async move {
    //         match self {
    //             Self::Tray(tray) => {
    //                 if let Tray::Base(base) = tray {
    //                     Ok(read(base))
    //                 } else {
    //                     Err(tray.wrong_variant("Base"))?
    //                 }
    //             }
    //             Self::Leaf(leaf) => leaf.read(read),
    //             Self::Ploy(ploy) => ploy.solve().await?.read(read).await,
    //             Self::Wing(wing) => wing.solve().await?.read(read).await,
    //         }
    //     })
    // }

    // pub fn base(&self) -> GraphFuture<Result<T>> {
    //     Box::pin(async move {
    //         match self {
    //             Self::Tray(tray) => match tray {
    //                 Tray::Base(base) => Ok(base.clone()),
    //                 tray => Err(tray.wrong_variant("Base"))?,
    //             },
    //             Self::Leaf(leaf) => leaf.read(|base| base.clone()),
    //             Self::Ploy(ploy) => ploy.solve().await?.base().await,
    //             Self::Wing(wing) => wing.solve().await?.base().await,
    //         }
    //     })
    // }

    // pub fn depend(&self) -> GraphFuture<Result<()>> {
    //     Box::pin(async move {
    //         match self {
    //             Self::Tray(_) => Ok(()),
    //             Self::Leaf(leaf) => leaf.read(|_| ()),
    //             Self::Ploy(ploy) => ploy.solve().await?.depend().await,
    //             Self::Wing(wing) => wing.solve().await?.depend().await,
    //         }
    //     })
    // }
}

impl<T: Payload> Backed for Hub<T> {
    fn backed(&self, back: &Back) -> Result<Self> {
        match self {
            Self::Tray(tray) => Ok(Self::Tray(tray.clone())),
            Self::Leaf(leaf) => Ok(Self::Leaf(leaf.backed(back)?)),
            Self::Ploy(ploy) => Ok(Self::Ploy(ploy.backed(back)?)),
            Self::Wing(wing) => Ok(Self::Wing(wing.backed(back)?)),
        }
    }
}

// impl<T: Payload> Default for Hub<T> {
//     fn default() -> Self {
//         Self::Tray(Tray::Base(T::default()))
//     }
// }

impl<T> Default for Hub<T> {
    fn default() -> Self {
        Self::Tray(Tray::None)
        // Self::Tray(Tray::Path(Path::Hash(0)))
    }
}

pub trait SolveDown<T>
where
    T: 'static + Payload,
{
    /// Solve down to the given graph rank.
    fn down(&self, rank: u16) -> impl Future<Output = Result<Vec<Hub<T>>>> + IsSend;
}

impl<T: Payload> SolveDown<T> for Vec<Hub<T>> {
    async fn down(&self, rank: u16) -> Result<Vec<Hub<T>>> {
        let mut out = vec![];
        for hub in self {
            out.push(hub.down(rank).await?);
        }
        Ok(out)
    }
}

// impl<T: Payload> Digest for Vec<Hub<T>> {
//     fn digest<H: Hasher>(&self, state: &mut H) {
//         for hub in self {
//             hub.digest(state);
//         }
//     }
// }

impl<T: Payload> Digest for Option<T> {
    fn digest<H: Hasher>(&self, state: &mut H) {
        if let Some(x) = self {
            x.digest(state);
        }
    }
}

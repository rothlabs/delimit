use super::*;
use anyhow::anyhow;
use get::*;
use thiserror::Error;
use view::*;

mod convert;
mod edit;
mod get;
mod hydrate;
mod import;
mod view;

#[derive(Error, Debug)]
pub enum Error {
    #[error("not ploy")]
    NotPloy,
    #[error("not ploy or leaf")]
    NotNode,
    // #[error(transparent)]
    // Tray(#[from] tray::Error),
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
    pub fn main(&self) -> Result<Apex> {
        match self {
            Self::Ploy(ploy) => ploy.main(),
            _ => Err(Error::NotPloy)?,
        }
    }

    pub fn get<'a>(&self, aim: impl Into<Aim<'a>>) -> Result<Apex> {
        match self {
            Self::Ploy(ploy) => {
                let mut get = Get::new(aim.into());
                ploy.adapt(&mut get)?;
                get.apex()
            }
            _ => Err(Error::NotPloy)?,
        }
    }

    pub fn imports(&self) -> Result<Vec<Import>> {
        match self {
            Self::Ploy(ploy) => ploy.solve(Task::Imports)?.imports(),
            _ => Err(Error::NotPloy)?,
        }
    }

    /// Insert apex into new Lake.
    pub fn lake(&self) -> Result<Lake> {
        let mut lake = Lake::new();
        lake.insert("root", self)?;
        Ok(lake)
    }

    /// Get hash digest number of apex.
    pub fn digest(&self) -> Result<u64> {
        match self {
            Self::Leaf(leaf) => leaf.solve(Task::Hash),
            Self::Ploy(ploy) => ploy.solve(Task::Hash),
            _ => Err(Error::NotNode)?,
        }?
        .u64()
    }

    /// Get serial string of apex.
    pub fn serial(&self) -> Result<String> {
        match self {
            Self::Tray(tray) => tray.serial(),
            Self::Leaf(leaf) => leaf.solve(Task::Serial),
            Self::Ploy(ploy) => ploy.solve(Task::Serial),
        }?
        .string()
    }

    /// Get stems of apex.
    pub fn all(&self) -> Result<Vec<Apex>> {
        match self {
            Self::Ploy(ploy) => {
                let mut all = All { apexes: vec![] };
                ploy.adapt(&mut all)?;
                Ok(all.apexes)
            }
            _ => Err(Error::NotPloy)?,
        }
    }

    /// Replace stems according to the Trade deal.
    pub fn adapt(&self, deal: &mut dyn Deal) -> Result<()> {
        if let Self::Ploy(ploy) = self {
            ploy.adapt(deal)?;
        }
        Ok(())
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
    pub fn tray(&self) -> Result<Tray> {
        match self {
            Self::Tray(tray) => Ok(tray.clone()),
            Self::Leaf(leaf) => leaf.tray(),
            Self::Ploy(ploy) => ploy.main()?.tray(),
        }
    }

    // pub fn set_rank(&mut self) -> Result<()> {
    //     match self {
    //         Self::Ploy(ploy) => {
    //             let apex = ploy.main()?;
    //             ploy.set_rank(apex.find_rank(1)?);
    //             // ploy.clear
    //             Ok(())
    //         },
    //         _ => Ok(())
    //     }
    // }

    // pub fn find_rank(&self, rank: usize) -> Result<usize> {
    //     match self {
    //         Self::Leaf(_) => Ok(rank),
    //         Self::Ploy(ploy) => ploy.main()?.find_rank(rank + 1),
    //         _ => Err(anyhow!("tried to find rank for tray"))?
    //     }
    // }

    /// Run Trade deal with this apex as input.
    pub fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
        deal.one(key, self)
    }

    /// Get rank of apex. Rank 1 apexes produce leaf apexes.
    pub fn rank(&self) -> Option<u64> {
        match self {
            Self::Ploy(ploy) => ploy.rank(),
            _ => None,
        }
    }

    /// Solve down to the given graph rank.
    pub fn down(&self, target: u64) -> Result<Apex> {
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

    /// Read tray of apex.
    pub fn read<T, F: FnOnce(&Tray) -> T>(&self, read: F) -> Result<T> {
        match self {
            Self::Tray(bare) => Ok(read(bare)),
            Self::Leaf(leaf) => leaf.read(read),
            Self::Ploy(ploy) => ploy.main()?.read(read),
        }
    }

    /// Make a View for reading Tray variants.
    pub fn view(&self) -> View {
        View { apex: self }
    }

    /// Clone String from Apex.
    pub fn string(&self) -> Result<String> {
        let tray = self.tray()?;
        match tray {
            Tray::String(value) => Ok(value),
            _ => Err(tray.wrong_variant("String"))?,
        }
    }

    /// u32 from Apex.
    pub fn u32(&self) -> Result<u32> {
        let tray = self.tray()?;
        match tray {
            Tray::U32(value) => Ok(value),
            _ => Err(tray.wrong_variant("u32"))?,
        }
    }

    /// i32 from Apex.
    pub fn i32(&self) -> Result<i32> {
        let tray = self.tray()?;
        match tray {
            Tray::I32(value) => Ok(value),
            _ => Err(tray.wrong_variant("i32"))?,
        }
    }
}

impl TryBacked for Apex {
    type Out = Self;
    /// New backed apex.
    fn backed(&self, back: &Back) -> Result<Self> {
        match self {
            Self::Tray(bare) => Ok(Self::Tray(bare.clone())),
            Self::Leaf(leaf) => Ok(Self::Leaf(leaf.backed(back)?)),
            Self::Ploy(ploy) => Ok(Self::Ploy(ploy.backed(back)?)),
        }
    }
}

impl Default for Apex {
    fn default() -> Self {
        Self::Tray(Tray::None)
    }
}

pub trait EngageApexes<'a> {
    /// Solve down to the given graph rank.
    fn down(&self, rank: u64) -> Result<Vec<Apex>>;
    /// Replace stems according to the Trade deal.
    fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()>;
}

impl<'a> EngageApexes<'a> for Vec<Apex> {
    fn down(&self, rank: u64) -> Result<Vec<Apex>> {
        self.iter().map(|x| x.down(rank)).collect()
    }
    fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
        deal.vec(key, self)
    }
}

#[derive(Debug)]
struct All {
    apexes: Vec<Apex>,
}

impl Deal for All {
    fn one(&mut self, _: &str, apex: &mut Apex) -> Result<()> {
        self.apexes.push(apex.clone());
        Ok(())
    }
    fn vec(&mut self, _: &str, apexes: &mut Vec<Apex>) -> Result<()> {
        self.apexes.extend(apexes.clone());
        Ok(())
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
        self.apexes.extend(map.all());
        Ok(())
    }
}

use super::*;
use anyhow::anyhow;
use thiserror::Error;
use view::*;

mod convert;
mod edit;
mod hydrate;
mod import;
mod view;
mod get;

#[derive(Error, Debug)]
pub enum Error {
    #[error("not ploy")]
    NotPloy,
    #[error("not ploy or leaf")]
    NotNode,
    #[error("wrong tray (expected: {expected:?}, found: {found:?})")]
    WrongTray { expected: String, found: Tray },
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

    /// Get stem by key
    pub fn get<'a>(&self, aim: impl Into<Aim<'a>>) -> Result<Apex> {
        match self {
            Self::Ploy(ploy) => match aim.into() {
                Aim::Key(key) => {
                    let map = ploy.solve(Task::Map)?.map()?;
                    if let Some(apex) = map.get(&key) {
                        Ok(apex)
                    } else {
                        Err(anyhow!("key not in map: {}", key))?
                    }
                },
                Aim::Keys(keys) => {
                    let apex = ploy.solve(Task::Get(&keys[0]))?.apex();
                    if keys.len() > 1 {
                        apex?.get(&keys[1..])
                    } else {
                        apex
                    }
                }
                Aim::Index(index) => {
                    if let Some(apex) = ploy.solve(Task::All)?.apexes()?.get(index) {
                        Ok(apex.clone())
                    } else {
                        Err(solve::Error::from(aim::Error::IndexOutOfBounds(index)))?
                    }
                }
            },
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
                let mut all = All{apexes:vec![]};
                ploy.adapt(&mut all)?;
                Ok(all.apexes)
            },
            _ => Err(Error::NotPloy)?,
        }
    }

    /// Replace stems according to the Trade deal.
    pub fn trade(&self, deal: &mut dyn Deal) {
        if let Self::Ploy(ploy) = self {
            ploy.adapt(deal).ok();
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
    pub fn tray(&self) -> Result<Tray> {
        match self {
            Self::Tray(bare) => Ok(bare.clone()),
            Self::Leaf(leaf) => leaf.tray(),
            Self::Ploy(ploy) => ploy.main()?.tray(),
        }
    }

    /// Run Trade deal with this apex as input.
    pub fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
        deal.one(key, self)
    }

    // pub fn echo(&mut self, other: Self) -> Result<()> {
    //     match self {
    //         Self::Tray(tray) => {
    //             match other {
    //                 Self::Tray(tray_r) => *tray = tray_r,
    //                 _ => return Err(anyhow!("could not echo apex"))?
    //             }
    //         }
            
    //     }
    //     Ok(())
    // }

    /// Get rank of apex. Rank 1 apexes produce leaf apexes.
    pub fn rank(&self) -> Option<usize> {
        match self {
            Self::Ploy(ploy) => ploy.rank(),
            _ => None,
        }
    }

    /// Solve down to the given graph rank.
    pub fn at(&self, target: usize) -> Result<Apex> {
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

    // /// New backed apex.
    // pub fn backed(&self, back: &Back) -> Self {
    //     match self {
    //         Self::Tray(bare) => Self::Tray(bare.clone()),
    //         Self::Leaf(leaf) => Self::Leaf(leaf.backed(back)),
    //         // TODO: remove unwrap!
    //         Self::Ploy(ploy) => Self::Ploy(ploy.backed(back).unwrap()),
    //     }
    // }

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
            _ => Err(wrong_tray("u32", &tray))?,
        }
    }

    /// i32 from Apex.
    pub fn i32(&self) -> Result<i32> {
        let tray = self.tray()?;
        match tray {
            Tray::I32(value) => Ok(value),
            _ => Err(wrong_tray("i32", &tray))?,
        }
    }
}

impl Backed for Apex {
    /// New backed apex.
    fn backed(&self, back: &Back) -> Self {
        match self {
            Self::Tray(bare) => Self::Tray(bare.clone()),
            Self::Leaf(leaf) => Self::Leaf(leaf.backed(back)),
            // TODO: remove unwrap!
            Self::Ploy(ploy) => Self::Ploy(ploy.backed(back).unwrap()),
        }
    }
}

pub fn wrong_tray(expected: &str, found: &Tray) -> Error {
    Error::WrongTray {
        expected: expected.into(),
        found: found.clone(),
    }
}

impl Default for Apex {
    fn default() -> Self {
        Self::Tray(Tray::None)
    }
}

pub trait EngageApexes<'a> {
    /// Solve down to the given graph rank.
    fn at(&self, rank: usize) -> Result<Vec<Apex>>;
    /// Replace stems according to the Trade deal.
    fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()>;
}

impl<'a> EngageApexes<'a> for Vec<Apex> {
    fn at(&self, rank: usize) -> Result<Vec<Apex>> {
        self.iter().map(|x| x.at(rank)).collect()
    }
    fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
        deal.vec(key, self)
    }
}

// pub trait EngageApexes {
//     /// Solve down to the given graph rank.
//     fn at(&self, rank: usize) -> Result<Vec<Apex>>;
//     /// Replace stems according to the Trade deal.
//     fn deal(&self, deal: &dyn Trade) -> Self;
// }

// impl EngageApexes for Vec<Apex> {
//     fn at(&self, rank: usize) -> Result<Vec<Apex>> {
//         self.iter().map(|x| x.at(rank)).collect()
//     }
//     fn deal(&self, deal: &dyn Trade) -> Self {
//         self.iter().map(|x| x.deal(deal)).collect()
//     }
// }

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
        self.apexes.extend(map.values().cloned());
        Ok(())
    }
}

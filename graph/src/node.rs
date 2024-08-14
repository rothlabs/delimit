use super::*;
use std::result;

mod import;
mod convert;

pub type Result = result::Result<Node, Error>;

/// Primary graph part. 
#[derive(Clone, PartialEq, Hash, Serialize, Debug)]
#[serde(untagged)]
pub enum Node {
    Tray(Tray),
    Leaf(Leaf),
    Ploy(Ploy),
}

impl Node {
    pub fn none() -> Self {
        Self::default()
    }

    /// Run main node function. Will return lower rank node if successful.
    pub fn main(&self) -> Result {
        match self {
            Self::Ploy(ploy) => ploy.main(),
            _ => Err("not ploy")?,
        }
    }

    /// Insert node into new Lake.
    pub fn lake(&self) -> lake::Result {
        let mut lake = Lake::new();
        lake.insert("root", self)?;
        Ok(lake)
    }

    /// Get hash digest number of node.
    pub fn digest(&self) -> result::Result<u64, Error> {
        match self {
            Self::Tray(tray) => tray.digest(),
            Self::Leaf(leaf) => leaf.solve(Task::Hash),
            Self::Ploy(ploy) => ploy.solve(Task::Hash),
        }?
        .u64()
    }

    /// Get serial string of node.
    pub fn serial(&self) -> serial::Result {
        match self {
            Self::Tray(tray) => tray.serial(),
            Self::Leaf(leaf) => leaf.solve(Task::Serial),
            Self::Ploy(ploy) => ploy.solve(Task::Serial),
        }?
        .string()
    }

    /// Get stems of node.
    pub fn stems(&self) -> result::Result<Vec<Node>, Error> {
        match self {
            Self::Ploy(ploy) => ploy.solve(Task::Stems),
            _ => empty_nodes(),
        }?
        .nodes()
    }

    /// Replace stems according to the Trade deal.
    pub fn trade(&self, deal: &dyn Trade) {
        if let Self::Ploy(ploy) = self {
            ploy.adapt(Post::Trade(deal)).ok();
        }
    }

    /// Get path associated with node if any.
    pub fn path(&self) -> Option<Path> {
        match self {
            Self::Tray(tray) => tray.path(),
            Self::Leaf(leaf) => leaf.path(),
            Self::Ploy(ploy) => ploy.path(),
        }
    }

    /// Get tray of node. Will solve to lowest rank if needed.
    pub fn tray(&self) -> tray::Result {
        match self {
            Self::Tray(bare) => Ok(bare.clone()),
            Self::Leaf(leaf) => Ok(leaf.tray()),
            Self::Ploy(ploy) => ploy.main()?.tray(),
        }
    }

    /// Run Trade deal with this node as input.
    pub fn deal(&self, deal: &dyn Trade) -> Self {
        deal.trade(self)
    }

    /// Get rank of node. Rank 1 nodes produce leaf nodes.
    pub fn rank(&self) -> Option<usize> {
        match self {
            Self::Ploy(ploy) => ploy.rank(),
            _ => None,
        }
    }

    /// Solve down to the given graph rank.
    pub fn at(&self, target: usize) -> Result {
        let mut node = self.clone();
        let mut rank = node.rank();
        while let Some(current) = rank {
            if current > target {
                node = node.main()?;
                rank = node.rank();
            } else {
                rank = None;
            }
        }
        Ok(node)
    }

    /// New backed node.
    pub fn backed(&self, back: &Back) -> Self {
        match self {
            Self::Tray(bare) => Self::Tray(bare.clone()),
            Self::Leaf(leaf) => Self::Leaf(leaf.backed(back)),
            Self::Ploy(ploy) => Self::Ploy(ploy.backed(back)),
        }
    }

    /// Read contents of node.
    pub fn read<T, F: FnOnce(tray::ResultRef) -> T>(&self, read: F) -> T {
        match self {
            Self::Tray(bare) => read(Ok(bare)),
            Self::Leaf(leaf) => leaf.read_tray(read),
            Self::Ploy(ploy) => {
                if let Ok(node) = ploy.main() {
                    node.read(read)
                } else {
                    read(Err("failed to read ploy".into()))
                }
            }
        }
    }

    pub fn read_or_error<T, F: FnOnce(&Tray) -> T>(&self, read: F) -> result::Result<T, Error> {
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
    pub fn string(&self) -> result::Result<String, Error> {
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

impl Default for Node {
    fn default() -> Self {
        Self::Tray(Tray::None)
    }
}

pub trait EngageNodes {
    /// Solve down to the given graph rank.
    fn at(&self, rank: usize) -> result::Result<Vec<Node>, Error>;
    /// Replace stems according to the Trade deal.
    fn deal(&self, deal: &dyn Trade) -> Self;
}

impl EngageNodes for Vec<Node> {
    fn at(&self, rank: usize) -> result::Result<Vec<Node>, Error> {
        self.iter().map(|x| x.at(rank)).collect()
    }
    fn deal(&self, deal: &dyn Trade) -> Self {
        self.iter().map(|x| x.deal(deal)).collect()
    }
}
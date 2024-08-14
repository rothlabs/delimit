use super::*;
use std::result;

mod import;
mod convert;

pub type Result = result::Result<Node, Error>;

/// Contains a bare load, meta about a link, or the link itself.
#[derive(Debug, Clone, PartialEq, Serialize, Hash)]
#[serde(untagged)]
pub enum Node {
    Load(Load),
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
            Self::Load(load) => load.digest(),
            Self::Leaf(leaf) => leaf.solve(Task::Hash),
            Self::Ploy(ploy) => ploy.solve(Task::Hash),
        }?
        .u64()
    }

    /// Get serial string of node.
    pub fn serial(&self) -> serial::Result {
        match self {
            Self::Load(load) => load.serial(),
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
            Self::Load(load) => load.path(),
            Self::Leaf(leaf) => leaf.path(),
            Self::Ploy(ploy) => ploy.path(),
        }
    }

    /// Get payload of node. Will solve to lowest rank if needed.
    pub fn load(&self) -> load::Result {
        match self {
            Self::Load(bare) => Ok(bare.clone()),
            Self::Leaf(leaf) => Ok(leaf.load()),
            Self::Ploy(ploy) => ploy.main()?.load(),
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

    /// Read contents of node.
    pub fn read<T, F: FnOnce(load::ResultRef) -> T>(&self, read: F) -> T {
        match self {
            Self::Load(bare) => read(Ok(bare)),
            Self::Leaf(leaf) => leaf.read_load(read),
            Self::Ploy(ploy) => {
                if let Ok(node) = ploy.main() {
                    node.read(read)
                } else {
                    read(Err("failed to read ploy".into()))
                }
            }
        }
    }

    pub fn read_or_error<T, F: FnOnce(&Load) -> T>(&self, read: F) -> result::Result<T, Error> {
        self.read(|load| match load {
            Ok(value) => Ok(read(value)),
            _ => Err("nothing to read")?,
        })
    }
    pub fn read_string<T, F: FnOnce(&String) -> T>(&self, read: F) -> T {
        self.read(|load| match load {
            Ok(Load::String(value)) => read(value),
            _ => read(&"".into()),
        })
    }
    pub fn read_vu8<T, F: FnOnce(&Vec<u8>) -> T>(&self, read: F) -> T {
        self.read(|load| match load {
            Ok(Load::Vu8(value)) => read(value),
            _ => read(&vec![]),
        })
    }
    pub fn read_vu16<T, F: FnOnce(&Vec<u16>) -> T>(&self, read: F) -> T {
        self.read(|load| match load {
            Ok(Load::Vu16(value)) => read(value),
            _ => read(&vec![]),
        })
    }
    pub fn read_vf32<T, F: FnOnce(&Vec<f32>) -> T>(&self, read: F) -> T {
        self.read(|load| match load {
            Ok(Load::Vf32(value)) => read(value),
            _ => read(&vec![]),
        })
    }
    pub fn string(&self) -> result::Result<String, Error> {
        match self.load() {
            Ok(Load::String(value)) => Ok(value),
            _ => Err("not a string")?,
        }
    }
    pub fn u32(&self) -> u32 {
        match self.load() {
            Ok(Load::U32(value)) => value,
            _ => 0,
        }
    }
    pub fn i32(&self) -> i32 {
        match self.load() {
            Ok(Load::I32(value)) => value,
            _ => 0,
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::Load(Load::None) // (Empty::default())
    }
}

impl Backed for Node {
    fn backed(&self, back: &Back) -> Self {
        match self {
            Self::Load(bare) => Self::Load(bare.clone()),
            Self::Leaf(leaf) => Self::Leaf(leaf.backed(back)),
            Self::Ploy(ploy) => Self::Ploy(ploy.backed(back)),
        }
    }
}

pub trait TradeNode {
    /// Trade nodes for others via base.
    fn deal(&self, base: &dyn Trade) -> Self;
}

impl TradeNode for Vec<Node> {
    fn deal(&self, base: &dyn Trade) -> Self {
        self.iter().map(|x| x.deal(base)).collect()
    }
}

pub trait SolveDown {
    /// Solve down to the given graph rank.
    fn at(&self, rank: usize) -> result::Result<Vec<Node>, Error>;
}

impl SolveDown for Vec<Node> {
    /// Solve down to the given graph rank.
    fn at(&self, rank: usize) -> result::Result<Vec<Node>, Error> {
        self.iter().map(|x| x.at(rank)).collect()
    }
}
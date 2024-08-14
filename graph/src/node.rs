use super::*;
// use load::Empty;
use serde::de::{MapAccess, Visitor};
// use serde_untagged::UntaggedEnumVisitor;
use std::{fmt, result};

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
    pub fn main(&self) -> Result {
        match self {
            Self::Ploy(ploy) => ploy.main(),
            _ => Err("not ploy")?,
        }
    }
    pub fn lake(&self) -> lake::Result {
        let mut lake = Lake::new();
        lake.insert("root", self)?;
        Ok(lake)
    }
    pub fn digest(&self) -> result::Result<u64, Error> {
        let hash_result = match self {
            Self::Load(load) => load.digest(),
            Self::Leaf(leaf) => leaf.solve(Task::Hash),
            Self::Ploy(ploy) => ploy.solve(Task::Hash),
        };
        match hash_result? {
            Tray::U64(hash) => Ok(hash),
            _ => Err("no hash")?,
        }
    }
    pub fn serial(&self) -> serial::Result {
        let serial_result = match self {
            Self::Load(load) => load.serial(),
            Self::Leaf(leaf) => leaf.solve(Task::Serial),
            Self::Ploy(ploy) => ploy.solve(Task::Serial),
        };
        match serial_result? {
            Tray::String(string) => Ok(string),
            _ => Err("no serial")?,
        }
    }
    pub fn stems(&self) -> result::Result<Vec<Node>, Error> {
        let stems = match self {
            Self::Ploy(ploy) => ploy.solve(Task::Stems),
            _ => empty_nodes(),
        };
        match stems? {
            Tray::Nodes(nodes) => Ok(nodes),
            _ => Err("no stems")?,
        }
    }
    pub fn trade(&self, deal: &dyn Trade) {
        if let Self::Ploy(ploy) = self {
            ploy.adapt(Post::Trade(deal)).ok();
        }
    }
    pub fn path(&self) -> Option<Path> {
        match self {
            Self::Load(load) => load.path(),
            Self::Leaf(leaf) => leaf.path(),
            Self::Ploy(ploy) => ploy.path(),
        }
    }
    pub fn load(&self) -> load::Result {
        match self {
            Self::Load(bare) => Ok(bare.clone()),
            Self::Leaf(leaf) => Ok(leaf.load()),
            Self::Ploy(ploy) => ploy.main()?.load(),
        }
    }
    pub fn deal(&self, deal: &dyn Trade) -> Self {
        deal.trade(self)
    }
    pub fn rank(&self) -> Option<usize> {
        match self {
            Self::Ploy(ploy) => ploy.rank(),
            _ => None,
        }
    }
    /// Solve down to the given rank.
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
    /// Reduce node rank down to specified number.
    fn at(&self, rank: usize) -> result::Result<Vec<Node>, Error>;
}

impl SolveDown for Vec<Node> {
    fn at(&self, rank: usize) -> result::Result<Vec<Node>, Error> {
        self.iter().map(|x| x.at(rank)).collect()
    }
}

impl From<Load> for Node {
    fn from(value: Load) -> Self {
        Node::Load(value)
    }
}

impl From<Leaf> for Node {
    fn from(leaf: Leaf) -> Self {
        Node::Leaf(leaf)
    }
}

impl From<Ploy> for Node {
    fn from(ploy: Ploy) -> Self {
        // TODO: find way to not query the node to get rank!
        let rank = match ploy.main() {
            Ok(node) => match node.rank() {
                Some(rank) => rank + 1,
                None => 1,
            },
            _ => 0,
        };
        Node::Ploy(ploy.ranked(rank))
    }
}

impl From<&Leaf> for Node {
    fn from(value: &Leaf) -> Self {
        Node::Leaf(value.clone())
    }
}

impl From<&Node> for Node {
    fn from(value: &Node) -> Self {
        value.clone()
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        Node::Load(Load::String(value.to_owned()))
    }
}

impl From<u32> for Node {
    fn from(value: u32) -> Self {
        Node::Load(Load::U32(value))
    }
}

impl From<i32> for Node {
    fn from(value: i32) -> Self {
        Node::Load(Load::I32(value))
    }
}

impl From<Vec<u8>> for Node {
    fn from(value: Vec<u8>) -> Self {
        Node::Leaf(Leaf::new(Load::Vu8(value)))
    }
}

impl From<Vec<u16>> for Node {
    fn from(value: Vec<u16>) -> Self {
        Node::Leaf(Leaf::new(Load::Vu16(value)))
    }
}

impl From<Vec<f32>> for Node {
    fn from(value: Vec<f32>) -> Self {
        Node::Leaf(Leaf::new(Load::Vf32(value)))
    }
}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // UntaggedEnumVisitor::new()
        //     .map(|map| map.deserialize().map(Node::Load))
        //     .deserialize(deserializer)
        deserializer.deserialize_map(NodeVisitor)
    }
}

struct NodeVisitor;

impl<'de> Visitor<'de> for NodeVisitor {
    type Value = Node;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("enum node form")
    }
    fn visit_map<A>(self, mut map: A) -> result::Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        if let Some(key) = map.next_key()? {
            let node = match key {
                NodeType::None => Node::none(),
                NodeType::Hash => Node::Load(Load::Path(Path::Hash(map.next_value()?))),
                NodeType::World => Node::Load(Load::Path(Path::World(map.next_value()?))),
                NodeType::Local => Node::Load(Load::Path(Path::Local(map.next_value()?))),
                NodeType::Upper => Node::Load(Load::Path(Path::Upper(map.next_value()?))),
                NodeType::String => Node::Load(Load::String(map.next_value()?)),
                NodeType::U8 => Node::Load(Load::U8(map.next_value()?)),
                NodeType::U16 => Node::Load(Load::U16(map.next_value()?)),
                NodeType::U32 => Node::Load(Load::U32(map.next_value()?)),
                NodeType::U64 => Node::Load(Load::U64(map.next_value()?)),
                NodeType::I8 => Node::Load(Load::I8(map.next_value()?)),
                NodeType::I16 => Node::Load(Load::I16(map.next_value()?)),
                NodeType::I32 => Node::Load(Load::I32(map.next_value()?)),
                NodeType::I64 => Node::Load(Load::I64(map.next_value()?)),
                NodeType::F32 => Node::Load(Load::F32(map.next_value()?)),
                NodeType::F64 => Node::Load(Load::F64(map.next_value()?)),
                NodeType::Vu8 => Node::Load(Load::Vu8(map.next_value()?)),
                NodeType::Vu16 => Node::Load(Load::Vu16(map.next_value()?)),
                NodeType::Vu32 => Node::Load(Load::Vu32(map.next_value()?)),
                NodeType::Vu64 => Node::Load(Load::Vu64(map.next_value()?)),
                NodeType::Vi8 => Node::Load(Load::Vi8(map.next_value()?)),
                NodeType::Vi16 => Node::Load(Load::Vi16(map.next_value()?)),
                NodeType::Vi32 => Node::Load(Load::Vi32(map.next_value()?)),
                NodeType::Vi64 => Node::Load(Load::Vi64(map.next_value()?)),
                NodeType::Vf32 => Node::Load(Load::Vf32(map.next_value()?)),
                NodeType::Vf64 => Node::Load(Load::Vf64(map.next_value()?)),
            };
            Ok(node)
        } else {
            Ok(Node::none())
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum NodeType {
    None,
    Hash,
    World,
    Local,
    Upper,
    String,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Vu8,
    Vu16,
    Vu32,
    Vu64,
    Vi8,
    Vi16,
    Vi32,
    Vi64,
    Vf32,
    Vf64,
}

// pub fn serial(&self, serial: &mut Serial) -> serial::Result {
//     if serial.contains(&self.meta()) {
//         return Ok(());
//     }
//     match self {
//         Self::Leaf(leaf) => leaf.serial(serial),
//         Self::Ploy(ploy) => ploy.serial(serial),
//         _ => Ok(()),
//     }
// }

// fn no_node<S>(serializer: S) -> result::Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     serializer.serialize_str(&r#"{"n":"n"}"#)
// }

// impl<'de> Deserialize<'de> for Node {
//     fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         const VARIANTS: &[&str] = &["Meta", "Load", "Leaf", "Ploy"];
//         // deserializer.deserialize_any(visitor)
//         deserializer.deserialize_enum("Node", VARIANTS, NodeVisitor)
//     }
// }

// #[derive(Deserialize)]
// // #[serde(variant_identifier)]
// enum NodeIdentifier {
//     Meta,
//     Load,
//     Leaf,
//     Ploy,
// }

// struct NodeVisitor;

// impl<'de> Visitor<'de> for NodeVisitor {
//     type Value = Node;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str("enum node form")
//     }

//     fn visit_enum<A>(self, data: A) -> result::Result<Node, A::Error>
//     where
//         A: de::EnumAccess<'de>,
//     {
//         let (identifier, variant) = data.variant()?;
//         match identifier {
//             NodeIdentifier::Meta => Ok(Node::Meta(variant.newtype_variant()?)),
//             NodeIdentifier::Load => Ok(Node::Load(variant.newtype_variant()?)),
//             NodeIdentifier::Leaf => Ok(Node::Meta(variant.newtype_variant()?)),
//             NodeIdentifier::Ploy => Ok(Node::Meta(variant.newtype_variant()?)),
//         }
//     }
// }

// match task {
//     // Task::Main => Ok(Self {
//     //     rank: self.rank - 1,
//     //     form: self.form.solve_form(task)?,
//     // }
//     // .into()),
// }

// pub fn solve_form(&self, _: Task) -> result::Result<Form, Error> {
//     match self {
//         Self::Meta(_) => Err("not a ploy")?,
//         Self::Load(_) => Err("not a ploy")?,
//         Self::Leaf(_) => Err("not a ploy")?,
//         Self::Ploy(ploy) => Ok(ploy.query().main()?),
//     }
// }
// pub fn solve(&self, task: Task) -> solve::Result {
//     match self {
//         Self::Meta(_) => Err("not a ploy")?,
//         Self::Load(_) => Err("not a ploy")?,
//         Self::Leaf(_) => Err("not a ploy")?,
//         Self::Ploy(ploy) => ploy.solve(task),
//     }
// }
// pub fn alter(&self, post: Post) -> adapt::Result {
//     match self {
//         Self::Meta(_) => Err("not a ploy")?,
//         Self::Load(_) => Err("not a ploy")?,
//         Self::Leaf(_) => Err("not a ploy")?,
//         Self::Ploy(ploy) => ploy.adapt(post),
//     }
// }

use super::*;
// use serde::de::{MapAccess, Visitor};
use serde_untagged::UntaggedEnumVisitor;
use std::result;

pub type Result = result::Result<Node, Error>;

/// Contains a bare load, meta about a link, or the link itself.
#[derive(Debug, Clone, PartialEq, Serialize)]
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
    /// Run main node function. Returns a new node at lower level.
    pub fn main(&self) -> Result {
        match self {
            Self::Ploy(ploy) => ploy.main(),
            _ => Err("not ploy".into()),
        }
    }
    pub fn lake(&self) -> lake::Result {
        let mut lake = Lake::new();
        lake.root(self.serial()?);
        Ok(lake)
    }
    pub fn serial(&self) -> serial::Result {
        match self.solve(Task::Serial)? {
            Tray::String(string) => Ok(string),
            _ => Err("not serialized".into()),
        }
    }
    pub fn path(&self) -> Path {
        match self {
            Self::Load(load) => load.path(),
            Self::Leaf(leaf) => leaf.path(),
            Self::Ploy(ploy) => ploy.path(),
        }
    }
    /// Solve graph down and clone out `Load` at lowest level
    pub fn load(&self) -> load::Result {
        match self {
            Self::Load(bare) => Ok(bare.clone()),
            Self::Leaf(leaf) => Ok(leaf.load()),
            Self::Ploy(ploy) => ploy.main()?.load(),
        }
    }
    pub fn trade(&self, base: &dyn Trade) -> Self {
        base.trade(self)
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
            _ => Err("nothing to read".into()),
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
    pub fn read_vf64<T, F: FnOnce(&Vec<f64>) -> T>(&self, read: F) -> T {
        self.read(|load| match load {
            Ok(Load::Vf64(value)) => read(value),
            _ => read(&vec![]),
        })
    }
    pub fn string(&self) -> result::Result<String, Error> {
        match self.load() {
            Ok(Load::String(value)) => Ok(value),
            _ => Err("not a string".into()),
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
    pub fn f64(&self) -> f64 {
        match self.load() {
            Ok(Load::F64(value)) => value,
            _ => 0.,
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::Load(Load::None)
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
    fn trade(&self, base: &dyn Trade) -> Self;
}

impl TradeNode for Vec<Node> {
    fn trade(&self, base: &dyn Trade) -> Self {
        self.iter().map(|x| x.trade(base)).collect()
    }
}

impl Solve for Node {
    fn solve(&self, task: Task) -> solve::Result {
        match self {
            Self::Ploy(ploy) => ploy.solve(task),
            _ => Err("not a ploy".into()),
        }
    }
}

impl AdaptInner for Node {
    fn adapt(&self, post: Post) -> adapt::Result {
        match self {
            Self::Ploy(ploy) => ploy.adapt(post),
            _ => Err("not a ploy".into()),
        }
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

impl From<String> for Node {
    fn from(value: String) -> Self {
        Node::Load(Load::String(value))
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

impl From<f64> for Node {
    fn from(value: f64) -> Self {
        Node::Leaf(Leaf::new(Load::F64(value)))
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
        UntaggedEnumVisitor::new()
            .map(|map| map.deserialize().map(Node::Load))
            .deserialize(deserializer)
        // deserializer.deserialize_map(NodeVisitor)
    }
}

// struct NodeVisitor;

// impl<'de> Visitor<'de> for NodeVisitor {
//     type Value = Node;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str("enum node form")
//     }
//     fn visit_map<A>(self, mut map: A) -> result::Result<Self::Value, A::Error>
//     where
//         A: MapAccess<'de>,
//     {
//         if let Some(key) = map.next_key()? {
//             let node = match key {
//                 NodeType::Path => Node::Meta(Path {
//                     keys: map.next_value()?,
//                 }),
//                 NodeType::String => Node::Load(Load::String(map.next_value()?)),
//                 NodeType::U8 => Node::Load(Load::U8(map.next_value()?)),
//                 NodeType::U16 => Node::Load(Load::U16(map.next_value()?)),
//                 NodeType::U32 => Node::Load(Load::U32(map.next_value()?)),
//                 NodeType::I8 => Node::Load(Load::I8(map.next_value()?)),
//                 NodeType::I16 => Node::Load(Load::I16(map.next_value()?)),
//                 NodeType::I32 => Node::Load(Load::I32(map.next_value()?)),
//                 NodeType::F32 => Node::Load(Load::F32(map.next_value()?)),
//                 NodeType::F64 => Node::Load(Load::F64(map.next_value()?)),
//                 NodeType::Vu8 => Node::Load(Load::Vu8(map.next_value()?)),
//                 NodeType::Vu16 => Node::Load(Load::Vu16(map.next_value()?)),
//                 NodeType::Vu32 => Node::Load(Load::Vu32(map.next_value()?)),
//                 NodeType::Vf32 => Node::Load(Load::Vf32(map.next_value()?)),
//                 NodeType::Vf64 => Node::Load(Load::Vf64(map.next_value()?)),
//                 _ => Node::none(),
//             };
//             Ok(node)
//         } else {
//             Ok(Node::none())
//         }
//     }
// }

// #[derive(Deserialize)]
// #[serde(rename_all = "lowercase")]
// enum NodeType {
//     N,
//     Path,
//     String,
//     U8,
//     U16,
//     U32,
//     I8,
//     I16,
//     I32,
//     F32,
//     F64,
//     Vu8,
//     Vu16,
//     Vu32,
//     Vf32,
//     Vf64,
// }

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
//         Self::Meta(_) => Err("not a ploy".into()),
//         Self::Load(_) => Err("not a ploy".into()),
//         Self::Leaf(_) => Err("not a ploy".into()),
//         Self::Ploy(ploy) => Ok(ploy.query().main()?),
//     }
// }
// pub fn solve(&self, task: Task) -> solve::Result {
//     match self {
//         Self::Meta(_) => Err("not a ploy".into()),
//         Self::Load(_) => Err("not a ploy".into()),
//         Self::Leaf(_) => Err("not a ploy".into()),
//         Self::Ploy(ploy) => ploy.solve(task),
//     }
// }
// pub fn alter(&self, post: Post) -> adapt::Result {
//     match self {
//         Self::Meta(_) => Err("not a ploy".into()),
//         Self::Load(_) => Err("not a ploy".into()),
//         Self::Leaf(_) => Err("not a ploy".into()),
//         Self::Ploy(ploy) => ploy.adapt(post),
//     }
// }

use super::*;
use std::{collections::HashMap, result};

pub type Result = result::Result<Lake, Error>;

/// Collection of serialized nodes.
/// Nodes are indexed by hash so they do not repeat.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Lake {
    roots: HashMap<Key, String>,
    nodes: HashMap<u64, String>,
    #[serde(skip)]
    atlas: Option<Box<dyn DeserializeNode>>,
}

impl Lake {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the atlas for deserializing entries into concrete nodes.
    pub fn atlas(&mut self, atlas: Box<dyn DeserializeNode>) -> &mut Self {
        self.atlas = Some(atlas);
        self
    }

    /// Insert graph into lake given root key and node.
    pub fn insert(&mut self, key: impl Into<Key>, node: &Node) -> adapt::Result {
        self.roots.insert(key.into(), node.serial()?);
        for node in &node.stems()? {
            self.insert_stem(node)?;
        }
        adapt_ok()
    }

    /// Insert stems recursively.
    fn insert_stem(&mut self, node: &Node) -> adapt::Result {
        self.nodes.insert(node.digest()?, node.serial()?);
        for node in &node.stems()? {
            self.insert_stem(node)?;
        }
        adapt_ok()
    }

    /// Get a root node by key.
    pub fn root(&self, key: impl Into<Key>) -> solve::Result {
        let serial = self.roots.get(&key.into()).ok_or("Root not in Lake.")?;
        self.atlas
            .as_ref()
            .ok_or("No atlas.")?
            .deserialize(serial)?
            .tray()
            .ok()
    }

    /// Get a node by hash.
    pub fn get(&self, hash: u64) -> solve::Result {
        let serial = self.nodes.get(&hash).ok_or("Node not in Lake.")?;
        self.atlas
            .as_ref()
            .ok_or("No atlas.")?
            .deserialize(serial)?
            .tray()
            .ok()
    }
}

impl Trade for Lake {
    fn trade(&self, node: &Node) -> Node {
        if let Node::Load(Load::Path(Path::Hash(hash))) = node {
            if let Ok(Tray::Node(node)) = self.get(*hash) {
                return node;
            }
        }
        node.clone()
    }
}

// #[derive(Clone, Debug)]
// struct LakeTrade {
//     lake: Lake,
// }

// impl LakeTrade {
//     fn new(lake: Lake) -> Box<Self> {
//         Box::new(Self { lake })
//     }
// }

// impl Adapt for Lake {
//     fn adapt(&mut self, post: Post) -> adapt::Result {
//         match post {
//             Post::Trade(_) => adapt_ok(),
//             _ => no_adapter(post),
//         }
//     }
// }

// impl Solve for Lake {
//     fn solve(&self, task: Task) -> solve::Result {
//         match task {
//             Task::Stems => empty_nodes(),
//             Task::Serial => self.serial(),
//             _ => no_solver(),
//         }
//     }
// }

// /// Serialize the given node as the root of the lake.
// pub fn root(&mut self, node: &Node) -> adapt::Result {
//     self.root = node.serial()?;
//     adapt_ok()
// }
// /// Insert a node into the lake as hash-serial pair.
// pub fn insert(&mut self, node: &Node) -> adapt::Result {
//     self.nodes.insert(node.digest()?, node.serial()?);
//     adapt_ok()
// }

// /// Set the root serial.
// pub fn root(&mut self, root: String) -> &mut Self {
//     self.root = root;
//     self
// }

// /// Insert hash-serial pair
// pub fn insert(&mut self, hash: u64, serial: String) -> &mut Self {
//     self.serials.insert(hash, serial);
//     self
// }

// pub fn new(ploy: impl Into<Node>) -> Result {
//     let mut serials = HashMap::new();

//     Ok(Self {
//         root: "".into(),
//         serials,
//     })
// }

// impl Adapt for Lake {
//     fn adapt(&mut self, post: Post) -> adapt::Result {
//         match post {
//             // Post::Trade(_) => Ok(Gain::None),
//             // Post::Extend(nodes) => self.extend(nodes),
//             // Post::Import => self.import(),
//             _ => did_not_adapt(post),
//         }
//     }
// }

// impl Solve for Lake {
//     fn solve(&self, task: Task) -> solve::Result {
//         match task {
//             // Task::Stems => self.stems(),
//             // Task::Export => self.export(),
//             // Task::Find(regex) => self.find(&regex),
//             _ => did_not_solve(),
//         }
//     }
// }

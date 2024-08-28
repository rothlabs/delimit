use anyhow::anyhow;

use super::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SerialNode {
    pub imports: Vec<Import>,
    pub unit: String,
}

/// Collection of serialized apexes. Indexed by hash.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Lake {
    roots: HashMap<Key, SerialNode>,
    nodes: HashMap<u64, SerialNode>,
    #[serde(skip)]
    atlas: Option<Box<dyn DeserializeUnit>>,
}

impl Lake {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the atlas for deserializing entries into concrete apexes.
    pub fn atlas(&mut self, atlas: Box<dyn DeserializeUnit>) -> &mut Self {
        self.atlas = Some(atlas);
        self
    }

    /// Insert graph into lake given root key and apex.
    pub fn insert(&mut self, key: impl Into<Key>, apex: &Apex) -> adapt::Result {
        let serial = SerialNode {
            imports: apex.imports().unwrap_or_default(),
            unit: apex.serial()?,
        };
        self.roots.insert(key.into(), serial);
        for apex in &apex.stems().unwrap_or_default() {
            self.insert_stem(apex)?;
        }
        adapt_ok()
    }

    /// Insert stems recursively.
    fn insert_stem(&mut self, apex: &Apex) -> adapt::Result {
        if let Apex::Tray(_) = apex {
            return adapt_ok();
        }
        let serial = SerialNode {
            imports: apex.imports().unwrap_or_default(),
            unit: apex.serial()?,
        };
        self.nodes.insert(apex.digest()?, serial);
        for apex in &apex.stems().unwrap_or_default() {
            self.insert_stem(apex)?;
        }
        adapt_ok()
    }

    /// Grow a tree from the lake.
    pub fn tree(&self) -> Result<Apex, Error> {
        let root = self.root("root")?;
        self.grow(&root).ok();
        Ok(root)
    }

    fn grow(&self, apex: &Apex) -> Result<(), Error> {
        apex.trade(self);
        for apex in apex.stems()? {
            self.grow(&apex).ok();
        }
        Ok(())
    }

    /// Get a root apex by Key.
    fn root(&self, key: impl Into<Key>) -> GraphResult<Apex> {
        let serial = self
            .roots
            .get(&key.into())
            .ok_or(anyhow!("Root node not found."))?;
        self.atlas
            .as_ref()
            .ok_or(anyhow!("No atlas."))?
            .deserialize(serial)
    }

    /// Get a apex by hash.
    fn get(&self, hash: u64) -> GraphResult<Apex> {
        let serial = self.nodes.get(&hash).ok_or(anyhow!("Node not found."))?;
        self.atlas
            .as_ref()
            .ok_or(anyhow!("No atlas."))?
            .deserialize(serial)
    }
}

impl Trade for Lake {
    fn trade(&self, apex: &Apex) -> Apex {
        if let Apex::Tray(Tray::Path(Path::Hash(hash))) = apex {
            if let Ok(apex) = self.get(*hash) {
                return apex;
            }
        }
        apex.clone()
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
//             Task::Serial => self.serial(),
//             _ => no_solver(),
//         }
//     }
// }

// /// Serialize the given apex as the root of the lake.
// pub fn root(&mut self, apex: &Apex) -> adapt::Result {
//     self.root = apex.serial()?;
//     adapt_ok()
// }
// /// Insert a apex into the lake as hash-serial pair.
// pub fn insert(&mut self, apex: &Apex) -> adapt::Result {
//     self.apexes.insert(apex.digest()?, apex.serial()?);
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

// pub fn new(ploy: impl Into<Apex>) -> Result {
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
//             // Post::Extend(apexes) => self.extend(apexes),
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

use super::*;
use std::{collections::HashMap, result};

pub type Result = result::Result<Lake, Error>;

/// Collection of serialized apexes.
/// Apexes are indexed by hash so they do not repeat.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Lake {
    roots: HashMap<Key, String>,
    nodes: HashMap<u64, String>,
    #[serde(skip)]
    atlas: Option<Box<dyn DeserializeApex>>,
}

impl Lake {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the atlas for deserializing entries into concrete apexes.
    pub fn atlas(&mut self, atlas: Box<dyn DeserializeApex>) -> &mut Self {
        self.atlas = Some(atlas);
        self
    }

    /// Insert graph into lake given root key and apex.
    pub fn insert(&mut self, key: impl Into<Key>, apex: &Apex) -> adapt::Result {
        self.roots.insert(key.into(), apex.serial()?);
        for apex in &apex.stems()? {
            self.insert_stem(apex)?;
        }
        adapt_ok()
    }

    /// Insert stems recursively.
    fn insert_stem(&mut self, apex: &Apex) -> adapt::Result {
        self.nodes.insert(apex.digest()?, apex.serial()?);
        for apex in &apex.stems()? {
            self.insert_stem(apex)?;
        }
        adapt_ok()
    }

    /// Get a root apex by Key.
    pub fn root(&self, key: impl Into<Key>) -> solve::Result {
        let serial = self
            .roots
            .get(&key.into())
            .ok_or("Root node not in Lake.")?;
        self.atlas
            .as_ref()
            .ok_or("No atlas.")?
            .deserialize(serial)?
            .gain()
    }

    /// Get a apex by hash.
    pub fn get(&self, hash: u64) -> solve::Result {
        let serial = self.nodes.get(&hash).ok_or("Node not in Lake.")?;
        self.atlas
            .as_ref()
            .ok_or("No atlas.")?
            .deserialize(serial)?
            .gain()
    }
}

impl Trade for Lake {
    fn trade(&self, apex: &Apex) -> Apex {
        if let Apex::Tray(Tray::Path(Path::Hash(hash))) = apex {
            if let Ok(Gain::Apex(apex)) = self.get(*hash) {
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
//             Task::Stems => empty_apexes(),
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

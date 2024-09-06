use anyhow::anyhow;

use super::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Serial {
    pub imports: Vec<Import>,
    pub unit: String,
}

/// Collection of serialized hubes, indexed by hash.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Lake {
    roots: HashMap<Key, Serial>,
    nodes: HashMap<u64, Serial>,
    #[serde(skip)]
    atlas: Option<Box<dyn DeserializeUnit>>,
    #[serde(skip)]
    back: Option<Back>,
}

impl Lake {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the atlas for deserializing entries into concrete hubes.
    pub fn atlas(&mut self, atlas: Box<dyn DeserializeUnit>) -> &mut Self {
        self.atlas = Some(atlas);
        self
    }

    /// Insert graph into lake given root key and hub.
    pub fn insert<T: Payload>(&mut self, key: impl Into<Key>, hub: &Hub<T>) -> Result<()> {
        let serial = Serial {
            imports: hub.imports().unwrap_or_default(),
            unit: hub.serial()?,
        };
        self.roots.insert(key.into(), serial);
        for hub in &hub.all().unwrap_or_default() {
            self.insert_stem(hub)?;
        }
        Ok(())
    }

    /// Insert stems recursively.
    fn insert_stem<T: Payload>(&mut self, hub: &Hub<T>) -> Result<()> {
        if let Hub::Tray(_) = hub {
            return Ok(());
        }
        let serial = Serial {
            imports: hub.imports().unwrap_or_default(),
            unit: hub.serial()?,
        };
        self.nodes.insert(hub.digest()?, serial);
        for hub in &hub.all().unwrap_or_default() {
            self.insert_stem(hub)?;
        }
        Ok(())
    }

    /// Grow a tree from the lake.
    pub fn tree(&mut self) -> Result<Hub> {
        let root = self.root("root")?;
        self.grow(&root).ok();
        Ok(root)
    }

    fn grow(&mut self, hub: &Hub) -> Result<()> {
        hub.adapt(self)?;
        for hub in hub.all()? {
            self.grow(&hub).ok();
        }
        Ok(())
    }

    /// Get a root hub by Key.
    fn root(&self, key: impl Into<Key>) -> Result<Hub> {
        let serial = self
            .roots
            .get(&key.into())
            .ok_or(anyhow!("Root node not found."))?;
        self.atlas
            .as_ref()
            .ok_or(anyhow!("No atlas."))?
            .deserialize(serial)
    }

    /// Get a hub by hash.
    fn get(&self, hash: u64) -> Result<Hub> {
        let serial = self.nodes.get(&hash).ok_or(anyhow!("Node not found."))?;
        self.atlas
            .as_ref()
            .ok_or(anyhow!("No atlas."))?
            .deserialize(serial)
    }

    /// Swap hash-hub for deserialized hub
    fn deal(&self, hub: &mut Hub) -> Result<()> {
        if let Hub::Tray(Tray::Path(Path::Hash(hash))) = hub {
            if let Ok(rhs) = self.get(*hash) {
                if let Some(back) = self.back.as_ref() {
                    *hub = rhs.backed(back)?;
                } else {
                    return no_back("Lake");
                }
            }
        }
        Ok(())
    }
}

impl Deal for Lake {
    fn back(&mut self, back: &Back) {
        self.back = Some(back.clone());
    }
    fn one(&mut self, _: &str, hub: &mut Hub) -> Result<()> {
        self.deal(hub)
    }
    fn vec(&mut self, _: &str, hubes: &mut Vec<Hub>) -> Result<()> {
        for hub in hubes {
            self.deal(hub)?;
        }
        Ok(())
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
        for (_, hub) in map.iter_mut() {
            self.deal(hub)?;
        }
        Ok(())
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
//     fn adapt(&mut self, post: Post) -> Result<Memo> {
//         match post {
//             Post::Trade(_) => adapt_ok(),
//             _ => no_adapter(post),
//         }
//     }
// }

// impl Solve for Lake {
//     fn solve(&self, task: Task) -> Result<Gain> {
//         match task {
//             Task::Serial => self.serial(),
//             _ => no_solver(),
//         }
//     }
// }

// /// Serialize the given hub as the root of the lake.
// pub fn root(&mut self, hub: &Hub) -> Result<Memo> {
//     self.root = hub.serial()?;
//     adapt_ok()
// }
// /// Insert a hub into the lake as hash-serial pair.
// pub fn insert(&mut self, hub: &Hub) -> Result<Memo> {
//     self.hubes.insert(hub.digest()?, hub.serial()?);
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

// pub fn new(ploy: impl Into<Hub>) -> Result {
//     let mut serials = HashMap::new();

//     Ok(Self {
//         root: "".into(),
//         serials,
//     })
// }

// impl Adapt for Lake {
//     fn adapt(&mut self, post: Post) -> Result<Memo> {
//         match post {
//             // Post::Trade(_) => Ok(Gain::None),
//             // Post::Extend(hubes) => self.extend(hubes),
//             // Post::Import => self.import(),
//             _ => did_not_adapt(post),
//         }
//     }
// }

// impl Solve for Lake {
//     fn solve(&self, task: Task) -> Result<Gain> {
//         match task {
//             // Task::Stems => self.stems(),
//             // Task::Export => self.export(),
//             // Task::Find(regex) => self.find(&regex),
//             _ => did_not_solve(),
//         }
//     }
// }

// struct OptionBack(Option<Back>);

// impl Serialize for OptionBack {
//     fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
//         serializer.serialize_none()
//     }
// }

// impl Deserialize for OptionBack {
//     fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
//         where
//             D: serde::Deserializer<'de> {
//         deserializer.deserialize_option(none)
//     }
// }

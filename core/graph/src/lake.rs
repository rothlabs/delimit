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

    pub fn serial(&self) -> Result<String> {
        let serial = serde_json::to_string(self)?;
        Ok(serial)
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
        for apex in &hub.all().unwrap_or_default() {
            apex.insert_in_lake(self)?;
        }
        Ok(())
    }

    /// Insert stems recursively.
    pub fn insert_stem<T: Payload>(&mut self, hub: &Hub<T>) -> Result<()> {
        if let Hub::Tray(_) = hub {
            return Ok(());
        }
        let serial = Serial {
            imports: hub.imports().unwrap_or_default(),
            unit: hub.serial()?,
        };
        self.nodes.insert(hub.get_hash()?, serial);
        for apex in &hub.all().unwrap_or_default() {
            apex.insert_in_lake(self)?;
        }
        Ok(())
    }

    /// Grow a tree from the lake.
    pub async fn tree(&mut self) -> Result<Apex> {
        // TODO build space/scope tree here so nodes do not need to assume a key!!!
        let root = self.root("root")?;
        let mut ring = Ring::new();
        root.grow_from_lake(self, &mut ring).ok();
        ring.react().await?;
        Ok(root)
    }

    pub fn grow<T: Payload>(&mut self, hub: &Hub<T>, ring: &mut Ring) -> Result<()> {
        ring.extend(hub.transient_set(self)?);
        for apex in hub.all()? {
            apex.grow_from_lake(self, ring).ok();
        }
        Ok(())
    }

    /// Get a root hub by Key.
    fn root(&self, key: impl Into<Key>) -> Result<Apex> {
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
    fn get(&self, hash: u64) -> Result<Apex> {
        let serial = self.nodes.get(&hash).ok_or(anyhow!("Node not found."))?;
        self.atlas
            .as_ref()
            .ok_or(anyhow!("No atlas."))?
            .deserialize(serial)
    }

    /// Swap hash-hub for deserialized hub
    fn deal<'a>(&self, view: impl Into<View<'a>>) -> Result<()> {
        let view: View<'a> = view.into();
        if let Some(Path::Hash(hash)) = view.path() {
            if let Ok(rhs) = self.get(*hash) {
                if let Some(back) = self.back.as_ref() {
                    view.set(rhs.backed(back)?)?;
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
    fn one(&mut self, _: &str, view: View) -> Result<()> {
        self.deal(view)
    }
    fn vec(&mut self, _: &str, view: ViewVec) -> Result<()> {
        for view in view.views() {
            self.deal(view)?;
        }
        Ok(())
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
        for (_, apex) in map.iter_mut() {
            self.deal(apex)?;
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

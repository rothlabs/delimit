use super::*;
use std::fmt;

pub trait ToSerial {
    /// Serialize to string.
    fn serial(&self) -> solve::Result;
}

impl<T> ToSerial for T
where
    T: Serialize,
{
    /// Serialize to string.
    fn serial(&self) -> solve::Result {
        serde_json::to_string(self)?.gain()
    }
}

pub trait DeserializeUnit: Debug + SendSync {
    /// Deserialize to `Apex` with concrete unit type.
    fn deserialize(&self, serial_node: &SerialNode) -> Result<Apex, anyhow::Error>;
}

pub trait ToHash {
    /// Hash to digest number.
    fn digest(&self, state: &mut UnitHasher) -> solve::Result;
}

impl<T> ToHash for T
where
    T: Hash,
{
    /// Hash to digest number.
    fn digest(&self, state: &mut UnitHasher) -> solve::Result {
        self.hash(state);
        state.finish().gain()
    }
}

pub struct UnitHasher(Box<dyn Hasher>);

impl Default for UnitHasher {
    fn default() -> Self {
        Self(Box::new(DefaultHasher::new()))
    }
}

impl Hasher for UnitHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes);
    }
    fn finish(&self) -> u64 {
        self.0.finish()
    }
}

impl fmt::Debug for UnitHasher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("UnitHasher").finish()
    }
}

// impl fmt::Display for Task<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Task::{}", 0)
//     }
// }

// impl Deref for UnitHasher {
//     type Target = Box<dyn Hasher>;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// pub trait SerializeGraph {
//     /// Serialize part of the graph.
//     fn serialize(&self) -> Result;
// }

// impl<T> SerializeGraph for Vec<T>
// where
//     T: SerializeGraph,
// {
//     fn serial(&self) -> Result {
//         for item in self {
//             item.serial(serial)?;
//         }
//         Ok(())
//     }
// }

// #[derive(Default, Serialize, Deserialize)]
// pub struct Serial {
//     pub parts: HashMap<Path, String>,
// }

// impl Serial {
//     pub fn new() -> Self {
//         Self::default()
//     }
//     pub fn contains(&self, meta: &Meta) -> bool {
//         self.parts.contains_key(&meta.path)
//     }
//     pub fn insert(&mut self, meta: &Meta, apex: String) {
//         self.parts.insert(meta.path.clone(), apex);
//     }
//     pub fn string(&self) -> result::Result<String, serde_json::Error> {
//         serde_json::to_string(self)
//     }
// }

// pub trait SerializeGraph {
//     fn serial(&self, serial: &mut Serial) -> Result;
// }

// impl<T> SerializeGraph for Vec<T>
// where
//     T: SerializeGraph,
// {
//     fn serial(&self, serial: &mut Serial) -> Result {
//         for item in self {
//             item.serial(serial)?;
//         }
//         Ok(())
//     }
// }

// pub trait SerializeGraphInner {
//     fn serial(&mut self, serial: &mut Serial, back: &Back) -> Result;
// }

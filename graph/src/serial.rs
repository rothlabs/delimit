use super::*;
use std::result;

pub type Result = result::Result<String, Error>;

pub trait ToSerial {
    fn serial(&self) -> solve::Result;
}

impl<T> ToSerial for T
where
    T: Serialize,
{
    /// Convert to a string.
    fn serial(&self) -> solve::Result {
        Ok(Tray::String(serde_json::to_string(self)?))
    }
}

dyn_clone::clone_trait_object!(DeserializeNode);
pub trait DeserializeNode: DynClone + Debug + SendSync {
    /// Deserialize as a node with concrete unit type.
    fn deserialize(&self, string: &str) -> node::Result;
}

pub trait ToHash {
    fn digest(&self) -> solve::Result;
}

impl<T> ToHash for T
where
    T: Hash,
{
    fn digest(&self) -> solve::Result {
        let mut state = DefaultHasher::new();
        self.hash(&mut state);
        Ok(Tray::U64(state.finish()))
    }
}

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
//     pub fn insert(&mut self, meta: &Meta, node: String) {
//         self.parts.insert(meta.path.clone(), node);
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

use super::*;

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
        Ok(Gain::String(serde_json::to_string(self)?))
    }
}

dyn_clone::clone_trait_object!(DeserializeApex);
pub trait DeserializeApex: DynClone + Debug + SendSync {
    /// Deserialize to `Apex` with concrete unit type.
    fn deserialize(&self, string: &str) -> Result<Apex, Error>;
}

pub trait ToHash {
    /// Hash to digest number.
    fn digest(&self) -> solve::Result;
}

impl<T> ToHash for T
where
    T: Hash,
{
    /// Hash to digest number.
    fn digest(&self) -> solve::Result {
        let mut state = DefaultHasher::new();
        self.hash(&mut state);
        state.finish().gain()
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

use crate::*;
use serde::de::{MapAccess, Visitor};
use std::fmt;

/// Work that holds a tray. The most simple work that allows read, write, and copy of the tray.
#[derive(Debug, Hash)]
pub struct Leaf<T> {
    tray: T,
    digest: Option<u64>,
}

impl<T> Leaf<T> 
where 
    T: Payload
{
    pub fn new(tray: T) -> Self {
        Self { tray, digest: None }
    }
    pub fn hub(self) -> Hub<T> {
        Hub::Leaf(link::Leaf::new(self.tray))
    }
}

impl<T> Leaf<T> 
where 
    T: Payload
{
    fn digest(&mut self) -> Result<Gain<T>> {
        if let Some(digest) = &self.digest {
            digest.gain()
        } else {
            let mut state = DefaultHasher::new();
            self.tray.hash(&mut state);
            let digest = state.finish();
            self.digest = Some(digest);
            digest.gain()
        }
    }
}

impl<T> FromItem for Leaf<T> {
    type Item = T;
    fn new(tray: Self::Item) -> Self {
        Self { tray, digest: None }
    }
}

impl<T> ToItem for Leaf<T> {
    type Item = T;
    fn item(&self) -> &Self::Item {
        &self.tray
    }
}

impl<T> MutTray<T> for Leaf<T> {
    fn tray(&mut self) -> &mut T {
        &mut self.tray
    }
}

impl<T> ReactMut for Leaf<T> {
    fn react(&mut self, _: &Id) -> react::Result {
        Ok(())
    }
}

impl<T> SolveMut for Leaf<T> 
where 
    T: Payload, //Hash + Serialize + Debug + SendSync,//Hash + Serialize + Debug
{
    type Out = T;
    fn solve(&mut self, task: Task) -> Result<Gain<T>> {
        match task {
            Task::Serial => self.serial(),
            Task::Hash => self.digest(),
            _ => task.no_handler(self),
        }
    }
}

impl<T> RebutMut for Leaf<T> {
    fn rebut(&mut self) -> Result<Ring> {
        Ok(Ring::new())
    }
}

impl<T> Clear for Leaf<T> {
    fn clear(&mut self) {
        self.digest = None;
    }
}

impl<T> Adapt for Leaf<T> {
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()> {
        Ok(())
    }
}

impl<T: Serialize> Serialize for Leaf<T> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.tray.serialize(serializer)
    }
}

// impl<'de, T> Deserialize<'de> for Leaf<T> {
//     fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         deserializer.deserialize_map(LeafVisitor)
//     }
// }

// struct LeafVisitor;

// impl<'de> Visitor<'de> for LeafVisitor {
//     type Value = Leaf;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str("enum leaf form")
//     }
//     fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
//     where
//         A: MapAccess<'de>,
//     {
//         if let Some(key) = map.next_key()? {
//             let leaf = match key {
//                 DataType::String => Leaf::new(Tray::String(map.next_value()?)),
//                 DataType::Bool => Leaf::new(Tray::Bool(map.next_value()?)),
//                 DataType::U8 => Leaf::new(Tray::U8(map.next_value()?)),
//                 DataType::U16 => Leaf::new(Tray::U16(map.next_value()?)),
//                 DataType::U32 => Leaf::new(Tray::U32(map.next_value()?)),
//                 DataType::U64 => Leaf::new(Tray::U64(map.next_value()?)),
//                 DataType::I8 => Leaf::new(Tray::I8(map.next_value()?)),
//                 DataType::I16 => Leaf::new(Tray::I16(map.next_value()?)),
//                 DataType::I32 => Leaf::new(Tray::I32(map.next_value()?)),
//                 DataType::I64 => Leaf::new(Tray::I64(map.next_value()?)),
//                 DataType::F32 => Leaf::new(Tray::F32(map.next_value()?)),
//                 DataType::F64 => Leaf::new(Tray::F64(map.next_value()?)),
//                 DataType::Vu8 => Leaf::new(Tray::Vu8(map.next_value()?)),
//                 DataType::Vu16 => Leaf::new(Tray::Vu16(map.next_value()?)),
//                 DataType::Vu32 => Leaf::new(Tray::Vu32(map.next_value()?)),
//                 DataType::Vu64 => Leaf::new(Tray::Vu64(map.next_value()?)),
//                 DataType::Vi8 => Leaf::new(Tray::Vi8(map.next_value()?)),
//                 DataType::Vi16 => Leaf::new(Tray::Vi16(map.next_value()?)),
//                 DataType::Vi32 => Leaf::new(Tray::Vi32(map.next_value()?)),
//                 DataType::Vi64 => Leaf::new(Tray::Vi64(map.next_value()?)),
//                 DataType::Vf32 => Leaf::new(Tray::Vf32(map.next_value()?)),
//                 DataType::Vf64 => Leaf::new(Tray::Vf64(map.next_value()?)),
//             };
//             Ok(leaf)
//         } else {
//             Ok(Leaf::new(Tray::None))
//         }
//     }
// }

// #[derive(Deserialize)]
// #[serde(rename_all = "lowercase")]
// enum DataType {
//     String,
//     Bool,
//     U8,
//     U16,
//     U32,
//     U64,
//     I8,
//     I16,
//     I32,
//     I64,
//     F32,
//     F64,
//     Vu8,
//     Vu16,
//     Vu32,
//     Vu64,
//     Vi8,
//     Vi16,
//     Vi32,
//     Vi64,
//     Vf32,
//     Vf64,
// }

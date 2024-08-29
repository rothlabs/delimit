use crate::*;
use serde::de::{MapAccess, Visitor};
use std::fmt;

/// Work that holds a tray. The most simple work that allows read, write, and copy of the tray.
#[derive(Debug, Hash)]
pub struct Leaf {
    tray: Tray,
    digest: Option<u64>,
}

impl Leaf {
    pub fn new(tray: Tray) -> Self {
        Self { tray, digest: None }
    }
    pub fn apex(self) -> Apex {
        Apex::Leaf(link::Leaf::new(self.tray))
    }
    fn digest(&mut self) -> solve::Result {
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

impl FromItem for Leaf {
    type Item = Tray;
    fn new(tray: Self::Item) -> Self {
        Self { tray, digest: None }
    }
}

impl ToItem for Leaf {
    type Item = Tray;
    fn item(&self) -> &Self::Item {
        &self.tray
    }
}

impl WriteTrayWork for Leaf {
    type Item = Tray;
    fn write_tray_work<T, F: FnOnce(&mut Self::Item) -> T>(
        &mut self,
        write: F,
    ) -> Result<T> {
        Ok(write(&mut self.tray))
    }
}

impl ReactMut for Leaf {
    fn react(&mut self, _: &Id) -> react::Result {
        Ok(())
    }
}

impl DoSolve for Leaf {
    fn do_solve(&mut self, task: Task) -> solve::Result {
        match task {
            Task::Serial => self.serial(),
            Task::Hash => self.digest(),
            _ => no_solver(self, task),
        }
    }
}

impl RebutMut for Leaf {
    fn rebut(&mut self) -> Ring {
        Ring::new()
    }
}

impl Clear for Leaf {
    fn clear(&mut self) {
        self.digest = None;
    }
}

impl Serialize for Leaf {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.tray.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Leaf {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(LeafVisitor)
    }
}

struct LeafVisitor;

impl<'de> Visitor<'de> for LeafVisitor {
    type Value = Leaf;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("enum leaf form")
    }
    fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        if let Some(key) = map.next_key()? {
            let leaf = match key {
                DataType::String => Leaf::new(Tray::String(map.next_value()?)),
                DataType::Bool => Leaf::new(Tray::Bool(map.next_value()?)),
                DataType::U8 => Leaf::new(Tray::U8(map.next_value()?)),
                DataType::U16 => Leaf::new(Tray::U16(map.next_value()?)),
                DataType::U32 => Leaf::new(Tray::U32(map.next_value()?)),
                DataType::U64 => Leaf::new(Tray::U64(map.next_value()?)),
                DataType::I8 => Leaf::new(Tray::I8(map.next_value()?)),
                DataType::I16 => Leaf::new(Tray::I16(map.next_value()?)),
                DataType::I32 => Leaf::new(Tray::I32(map.next_value()?)),
                DataType::I64 => Leaf::new(Tray::I64(map.next_value()?)),
                DataType::F32 => Leaf::new(Tray::F32(map.next_value()?)),
                DataType::F64 => Leaf::new(Tray::F64(map.next_value()?)),
                DataType::Vu8 => Leaf::new(Tray::Vu8(map.next_value()?)),
                DataType::Vu16 => Leaf::new(Tray::Vu16(map.next_value()?)),
                DataType::Vu32 => Leaf::new(Tray::Vu32(map.next_value()?)),
                DataType::Vu64 => Leaf::new(Tray::Vu64(map.next_value()?)),
                DataType::Vi8 => Leaf::new(Tray::Vi8(map.next_value()?)),
                DataType::Vi16 => Leaf::new(Tray::Vi16(map.next_value()?)),
                DataType::Vi32 => Leaf::new(Tray::Vi32(map.next_value()?)),
                DataType::Vi64 => Leaf::new(Tray::Vi64(map.next_value()?)),
                DataType::Vf32 => Leaf::new(Tray::Vf32(map.next_value()?)),
                DataType::Vf64 => Leaf::new(Tray::Vf64(map.next_value()?)),
            };
            Ok(leaf)
        } else {
            Ok(Leaf::new(Tray::None))
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum DataType {
    String,
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Vu8,
    Vu16,
    Vu32,
    Vu64,
    Vi8,
    Vi16,
    Vi32,
    Vi64,
    Vf32,
    Vf64,
}

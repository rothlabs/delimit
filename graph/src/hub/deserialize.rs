use super::*;
use serde::de::{MapAccess, Visitor};
use std::{fmt, result};

impl<'de> Deserialize<'de> for Hub {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(HubVisitor)
    }
}

struct HubVisitor;

impl<'de> Visitor<'de> for HubVisitor {
    type Value = Hub;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("enum hub form")
    }
    fn visit_map<A>(self, mut map: A) -> result::Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        if let Some(key) = map.next_key()? {
            let hub = match key {
                DataType::None => Hub::none(),
                DataType::Hash => Hub::Tray(Tray::Path(Path::Hash(map.next_value()?))),
                DataType::World => Hub::Tray(Tray::Path(Path::World(map.next_value()?))),
                DataType::Local => Hub::Tray(Tray::Path(Path::Local(map.next_value()?))),
                DataType::Upper => Hub::Tray(Tray::Path(Path::Upper(map.next_value()?))),
                DataType::String => Hub::Tray(Tray::String(map.next_value()?)),
                DataType::Bool => Hub::Tray(Tray::Bool(map.next_value()?)),
                DataType::U8 => Hub::Tray(Tray::U8(map.next_value()?)),
                DataType::U16 => Hub::Tray(Tray::U16(map.next_value()?)),
                DataType::U32 => Hub::Tray(Tray::U32(map.next_value()?)),
                DataType::U64 => Hub::Tray(Tray::U64(map.next_value()?)),
                DataType::I8 => Hub::Tray(Tray::I8(map.next_value()?)),
                DataType::I16 => Hub::Tray(Tray::I16(map.next_value()?)),
                DataType::I32 => Hub::Tray(Tray::I32(map.next_value()?)),
                DataType::I64 => Hub::Tray(Tray::I64(map.next_value()?)),
                DataType::F32 => Hub::Tray(Tray::F32(map.next_value()?)),
                DataType::F64 => Hub::Tray(Tray::F64(map.next_value()?)),
                DataType::Vu8 => Hub::Tray(Tray::Vu8(map.next_value()?)),
                DataType::Vu16 => Hub::Tray(Tray::Vu16(map.next_value()?)),
                DataType::Vu32 => Hub::Tray(Tray::Vu32(map.next_value()?)),
                DataType::Vu64 => Hub::Tray(Tray::Vu64(map.next_value()?)),
                DataType::Vi8 => Hub::Tray(Tray::Vi8(map.next_value()?)),
                DataType::Vi16 => Hub::Tray(Tray::Vi16(map.next_value()?)),
                DataType::Vi32 => Hub::Tray(Tray::Vi32(map.next_value()?)),
                DataType::Vi64 => Hub::Tray(Tray::Vi64(map.next_value()?)),
                DataType::Vf32 => Hub::Tray(Tray::Vf32(map.next_value()?)),
                DataType::Vf64 => Hub::Tray(Tray::Vf64(map.next_value()?)),
            };
            Ok(hub)
        } else {
            Ok(Hub::none())
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum DataType {
    None,
    Hash,
    World,
    Local,
    Upper,
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

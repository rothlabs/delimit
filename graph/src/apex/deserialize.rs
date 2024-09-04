use super::*;
use serde::de::{MapAccess, Visitor};
use std::{fmt, result};

impl<'de> Deserialize<'de> for Apex {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ApexVisitor)
    }
}

struct ApexVisitor;

impl<'de> Visitor<'de> for ApexVisitor {
    type Value = Apex;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("enum apex form")
    }
    fn visit_map<A>(self, mut map: A) -> result::Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        if let Some(key) = map.next_key()? {
            let apex = match key {
                DataType::None => Apex::none(),
                DataType::Hash => Apex::Tray(Tray::Path(Path::Hash(map.next_value()?))),
                DataType::World => Apex::Tray(Tray::Path(Path::World(map.next_value()?))),
                DataType::Local => Apex::Tray(Tray::Path(Path::Local(map.next_value()?))),
                DataType::Upper => Apex::Tray(Tray::Path(Path::Upper(map.next_value()?))),
                DataType::String => Apex::Tray(Tray::String(map.next_value()?)),
                DataType::Bool => Apex::Tray(Tray::Bool(map.next_value()?)),
                DataType::U8 => Apex::Tray(Tray::U8(map.next_value()?)),
                DataType::U16 => Apex::Tray(Tray::U16(map.next_value()?)),
                DataType::U32 => Apex::Tray(Tray::U32(map.next_value()?)),
                DataType::U64 => Apex::Tray(Tray::U64(map.next_value()?)),
                DataType::I8 => Apex::Tray(Tray::I8(map.next_value()?)),
                DataType::I16 => Apex::Tray(Tray::I16(map.next_value()?)),
                DataType::I32 => Apex::Tray(Tray::I32(map.next_value()?)),
                DataType::I64 => Apex::Tray(Tray::I64(map.next_value()?)),
                DataType::F32 => Apex::Tray(Tray::F32(map.next_value()?)),
                DataType::F64 => Apex::Tray(Tray::F64(map.next_value()?)),
                DataType::Vu8 => Apex::Tray(Tray::Vu8(map.next_value()?)),
                DataType::Vu16 => Apex::Tray(Tray::Vu16(map.next_value()?)),
                DataType::Vu32 => Apex::Tray(Tray::Vu32(map.next_value()?)),
                DataType::Vu64 => Apex::Tray(Tray::Vu64(map.next_value()?)),
                DataType::Vi8 => Apex::Tray(Tray::Vi8(map.next_value()?)),
                DataType::Vi16 => Apex::Tray(Tray::Vi16(map.next_value()?)),
                DataType::Vi32 => Apex::Tray(Tray::Vi32(map.next_value()?)),
                DataType::Vi64 => Apex::Tray(Tray::Vi64(map.next_value()?)),
                DataType::Vf32 => Apex::Tray(Tray::Vf32(map.next_value()?)),
                DataType::Vf64 => Apex::Tray(Tray::Vf64(map.next_value()?)),
            };
            Ok(apex)
        } else {
            Ok(Apex::none())
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

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
                ApexType::None => Apex::none(),
                ApexType::Hash => Apex::Tray(Tray::Path(Path::Hash(map.next_value()?))),
                ApexType::World => Apex::Tray(Tray::Path(Path::World(map.next_value()?))),
                ApexType::Local => Apex::Tray(Tray::Path(Path::Local(map.next_value()?))),
                ApexType::Upper => Apex::Tray(Tray::Path(Path::Upper(map.next_value()?))),
                ApexType::String => Apex::Tray(Tray::String(map.next_value()?)),
                ApexType::U8 => Apex::Tray(Tray::U8(map.next_value()?)),
                ApexType::U16 => Apex::Tray(Tray::U16(map.next_value()?)),
                ApexType::U32 => Apex::Tray(Tray::U32(map.next_value()?)),
                ApexType::U64 => Apex::Tray(Tray::U64(map.next_value()?)),
                ApexType::I8 => Apex::Tray(Tray::I8(map.next_value()?)),
                ApexType::I16 => Apex::Tray(Tray::I16(map.next_value()?)),
                ApexType::I32 => Apex::Tray(Tray::I32(map.next_value()?)),
                ApexType::I64 => Apex::Tray(Tray::I64(map.next_value()?)),
                ApexType::F32 => Apex::Tray(Tray::F32(map.next_value()?)),
                ApexType::F64 => Apex::Tray(Tray::F64(map.next_value()?)),
                ApexType::Vu8 => Apex::Tray(Tray::Vu8(map.next_value()?)),
                ApexType::Vu16 => Apex::Tray(Tray::Vu16(map.next_value()?)),
                ApexType::Vu32 => Apex::Tray(Tray::Vu32(map.next_value()?)),
                ApexType::Vu64 => Apex::Tray(Tray::Vu64(map.next_value()?)),
                ApexType::Vi8 => Apex::Tray(Tray::Vi8(map.next_value()?)),
                ApexType::Vi16 => Apex::Tray(Tray::Vi16(map.next_value()?)),
                ApexType::Vi32 => Apex::Tray(Tray::Vi32(map.next_value()?)),
                ApexType::Vi64 => Apex::Tray(Tray::Vi64(map.next_value()?)),
                ApexType::Vf32 => Apex::Tray(Tray::Vf32(map.next_value()?)),
                ApexType::Vf64 => Apex::Tray(Tray::Vf64(map.next_value()?)),
            };
            Ok(apex)
        } else {
            Ok(Apex::none())
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum ApexType {
    None,
    Hash,
    World,
    Local,
    Upper,
    String,
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

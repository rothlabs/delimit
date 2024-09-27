use super::*;
use serde::de::{MapAccess, Visitor};
use std::{fmt, marker::PhantomData, result};

impl<'de, T> Deserialize<'de> for Hub<T>
where
    T: Payload + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(HubVisitor { out: PhantomData })
    }
}

struct HubVisitor<T> {
    out: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for HubVisitor<T>
where
    T: 'static + Payload + Deserialize<'de>,
{
    type Value = Hub<T>;

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
                DataType::Base => Hub::Tray(Tray::Base(map.next_value()?)),
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
    Base,
}

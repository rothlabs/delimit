use super::*;
use serde::de::{MapAccess, Visitor};
use std::{fmt, result};

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(NodeVisitor)
    }
}

struct NodeVisitor;

impl<'de> Visitor<'de> for NodeVisitor {
    type Value = Node;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("enum node form")
    }
    fn visit_map<A>(self, mut map: A) -> result::Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        if let Some(key) = map.next_key()? {
            let node = match key {
                NodeType::None => Node::none(),
                NodeType::Hash => Node::Load(Load::Path(Path::Hash(map.next_value()?))),
                NodeType::World => Node::Load(Load::Path(Path::World(map.next_value()?))),
                NodeType::Local => Node::Load(Load::Path(Path::Local(map.next_value()?))),
                NodeType::Upper => Node::Load(Load::Path(Path::Upper(map.next_value()?))),
                NodeType::String => Node::Load(Load::String(map.next_value()?)),
                NodeType::U8 => Node::Load(Load::U8(map.next_value()?)),
                NodeType::U16 => Node::Load(Load::U16(map.next_value()?)),
                NodeType::U32 => Node::Load(Load::U32(map.next_value()?)),
                NodeType::U64 => Node::Load(Load::U64(map.next_value()?)),
                NodeType::I8 => Node::Load(Load::I8(map.next_value()?)),
                NodeType::I16 => Node::Load(Load::I16(map.next_value()?)),
                NodeType::I32 => Node::Load(Load::I32(map.next_value()?)),
                NodeType::I64 => Node::Load(Load::I64(map.next_value()?)),
                NodeType::F32 => Node::Load(Load::F32(map.next_value()?)),
                NodeType::F64 => Node::Load(Load::F64(map.next_value()?)),
                NodeType::Vu8 => Node::Load(Load::Vu8(map.next_value()?)),
                NodeType::Vu16 => Node::Load(Load::Vu16(map.next_value()?)),
                NodeType::Vu32 => Node::Load(Load::Vu32(map.next_value()?)),
                NodeType::Vu64 => Node::Load(Load::Vu64(map.next_value()?)),
                NodeType::Vi8 => Node::Load(Load::Vi8(map.next_value()?)),
                NodeType::Vi16 => Node::Load(Load::Vi16(map.next_value()?)),
                NodeType::Vi32 => Node::Load(Load::Vi32(map.next_value()?)),
                NodeType::Vi64 => Node::Load(Load::Vi64(map.next_value()?)),
                NodeType::Vf32 => Node::Load(Load::Vf32(map.next_value()?)),
                NodeType::Vf64 => Node::Load(Load::Vf64(map.next_value()?)),
            };
            Ok(node)
        } else {
            Ok(Node::none())
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum NodeType {
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
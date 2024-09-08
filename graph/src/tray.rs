use super::*;
use serde::Serialize;
use std::hash::{Hash, Hasher};

/// Payload value of graph parts.
/// Used as payload of `Leaf` and field values of `Node` and `Ploy`.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tray<T> {
    None,
    Path(Path),
    Item(T),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong variant (expected: {expected:?}, found: {found:?})")]
    WrongVariant { expected: String, found: String },
}

impl<T> Tray<T>
where
    T: Debug,
{
    pub fn wrong_variant(&self, expected: &str) -> Error {
        Error::WrongVariant {
            expected: expected.into(),
            found: format!("{:?}", self),
        }
    }
    pub fn path(&self) -> Option<&Path> {
        match self {
            Self::Path(path) => Some(path),
            _ => None,
        }
    }
}

impl<T: Hash> Hash for Tray<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Path(path) => path.hash(state),
            Self::Item(item) => item.hash(state),
            _ => (),
        }
    }
}

// impl Hash for Tray {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         match self {
//             Self::Path(path) => path.hash(state),
//             Self::String(string) => string.hash(state),
//             Self::U8(int) => int.hash(state),
//             Self::U16(int) => int.hash(state),
//             Self::U32(int) => int.hash(state),
//             Self::U64(int) => int.hash(state),
//             Self::I8(int) => int.hash(state),
//             Self::I16(int) => int.hash(state),
//             Self::I32(int) => int.hash(state),
//             Self::I64(int) => int.hash(state),
//             Self::F32(float) => float.to_bits().hash(state),
//             Self::F64(float) => float.to_bits().hash(state),
//             Self::Vu8(vec_int) => vec_int.hash(state),
//             Self::Vu16(vec_int) => vec_int.hash(state),
//             Self::Vu32(vec_int) => vec_int.hash(state),
//             Self::Vu64(vec_int) => vec_int.hash(state),
//             Self::Vi8(vec_int) => vec_int.hash(state),
//             Self::Vi16(vec_int) => vec_int.hash(state),
//             Self::Vi32(vec_int) => vec_int.hash(state),
//             Self::Vi64(vec_int) => vec_int.hash(state),
//             Self::Vf32(vec_float) => vec_float
//                 .iter()
//                 .map(|x| x.to_bits())
//                 .collect::<Vec<u32>>()
//                 .hash(state),
//             Self::Vf64(vec_float) => vec_float
//                 .iter()
//                 .map(|x| x.to_bits())
//                 .collect::<Vec<u64>>()
//                 .hash(state),
//             _ => (),
//         }
//     }
// }

use super::*;
use serde::Serialize;
use std::hash::{Hash, Hasher};
use std::result;

pub type Result = result::Result<Tray, Error>;
pub type ResultRef<'a> = result::Result<&'a Tray, AnyError>;

/// Payload value of graph parts.
/// Used as `Leaf` payload and field values of `Node` and `Ploy`.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tray {
    None,
    Path(Path),
    Bool(bool),
    String(String),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Vu8(Vec<u8>),
    Vu16(Vec<u16>),
    Vu32(Vec<u32>),
    Vu64(Vec<u64>),
    Vi8(Vec<i8>),
    Vi16(Vec<i16>),
    Vi32(Vec<i32>),
    Vi64(Vec<i64>),
    Vf32(Vec<f32>),
    Vf64(Vec<f64>),
}

impl Tray {
    pub fn path(&self) -> Option<&Path> {
        match self {
            Self::Path(path) => Some(path),
            _ => None,
        }
    }
}

impl Hash for Tray {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Path(path) => path.hash(state),
            Self::String(string) => string.hash(state),
            Self::U8(int) => int.hash(state),
            Self::U16(int) => int.hash(state),
            Self::U32(int) => int.hash(state),
            Self::U64(int) => int.hash(state),
            Self::I8(int) => int.hash(state),
            Self::I16(int) => int.hash(state),
            Self::I32(int) => int.hash(state),
            Self::I64(int) => int.hash(state),
            Self::F32(float) => float.to_bits().hash(state),
            Self::F64(float) => float.to_bits().hash(state),
            Self::Vu8(vec_int) => vec_int.hash(state),
            Self::Vu16(vec_int) => vec_int.hash(state),
            Self::Vu32(vec_int) => vec_int.hash(state),
            Self::Vu64(vec_int) => vec_int.hash(state),
            Self::Vi8(vec_int) => vec_int.hash(state),
            Self::Vi16(vec_int) => vec_int.hash(state),
            Self::Vi32(vec_int) => vec_int.hash(state),
            Self::Vi64(vec_int) => vec_int.hash(state),
            Self::Vf32(vec_float) => vec_float
                .iter()
                .map(|x| x.to_bits())
                .collect::<Vec<u32>>()
                .hash(state),
            Self::Vf64(vec_float) => vec_float
                .iter()
                .map(|x| x.to_bits())
                .collect::<Vec<u64>>()
                .hash(state),
            _ => (),
        }
    }
}

use super::*;
use serde::Serialize;
use std::result;

pub type Result = result::Result<Load, Error>;
pub type ResultRef<'a> = result::Result<&'a Load, Error>;

/// The Load enum allows Nodes to be handled with unknown payload datatype.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Load {
    String(String),
    U8(u8),
    U16(u16),
    U32(u32),
    I8(i8),
    I16(i16),
    I32(i32),
    F32(f32),
    F64(f64),
    Vu8(Vec<u8>),
    Vu16(Vec<u16>),
    Vu32(Vec<u32>),
    Vf32(Vec<f32>),
    Vf64(Vec<f64>),
}

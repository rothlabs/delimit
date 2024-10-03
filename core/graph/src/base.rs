use super::*;
use std::ops::Deref;

/// `Vec<f32>` for Graph. Includes required Hash implementation.
#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Vf32(pub Vec<f32>);

impl Hash for Vf32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0
            .iter()
            .map(|x| x.to_bits())
            .collect::<Vec<u32>>()
            .hash(state)
    }
}

impl Deref for Vf32 {
    type Target = Vec<f32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<f32>> for Vf32 {
    fn from(value: Vec<f32>) -> Self {
        Vf32(value)
    }
}

#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Vf64(pub Vec<f64>);

impl Hash for Vf64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0
            .iter()
            .map(|x| x.to_bits())
            .collect::<Vec<u64>>()
            .hash(state)
    }
}

impl Deref for Vf64 {
    type Target = Vec<f64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait CastSlice {
    fn slice<B>(&self) -> &[B]
    where
        B: bytemuck::AnyBitPattern;
}

impl CastSlice for Vec<u32> {
    fn slice<B>(&self) -> &[B]
        where
            B: bytemuck::AnyBitPattern {
        bytemuck::cast_slice(self)
    }
}

impl CastSlice for Vf32 {
    fn slice<B>(&self) -> &[B]
        where
            B: bytemuck::AnyBitPattern {
        bytemuck::cast_slice(&self.0)
    }
}
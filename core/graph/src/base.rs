use super::*;

pub trait HashGraph {
    fn hash_graph<H: Hasher>(&self, state: &mut H);
}

macro_rules! hash_base {
    ($ty:ty) => {
        impl HashGraph for $ty {
            fn hash_graph<H: Hasher>(&self, state: &mut H) {
                std::hash::Hash::hash(self, state);
            }
        }
    };
}

hash_base!(());
hash_base!(String);
hash_base!(u8);
hash_base!(u16);
hash_base!(u32);
hash_base!(u64);
hash_base!(Vec<u8>);
hash_base!(Vec<u16>);
hash_base!(Vec<u32>);
hash_base!(Vec<u64>);
hash_base!(i8);
hash_base!(i16);
hash_base!(i32);
hash_base!(i64);
hash_base!(Vec<i8>);
hash_base!(Vec<i16>);
hash_base!(Vec<i32>);
hash_base!(Vec<i64>);

impl HashGraph for f32 {
    fn hash_graph<H: Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.to_bits(), state);
    }
}

impl HashGraph for f64 {
    fn hash_graph<H: Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.to_bits(), state);
    }
}

impl HashGraph for Vec<f32> {
    fn hash_graph<H: Hasher>(&self, state: &mut H) {
        let slice: &[u8] = bytemuck::cast_slice(self);
        std::hash::Hash::hash(slice, state);
    }
}

impl HashGraph for Vec<f64> {
    fn hash_graph<H: Hasher>(&self, state: &mut H) {
        let slice: &[u8] = bytemuck::cast_slice(self);
        std::hash::Hash::hash(slice, state);
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

impl CastSlice for Vec<f32> {
    fn slice<B>(&self) -> &[B]
        where
            B: bytemuck::AnyBitPattern {
        bytemuck::cast_slice(self)
    }
}






// use std::ops::Deref;

// /// `Vec<f32>` for Graph. Includes required Hash implementation.
// #[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
// pub struct Vf32(pub Vec<f32>);

// impl Hash for Vf32 {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.0
//             .iter()
//             .map(|x| x.to_bits())
//             .collect::<Vec<u32>>()
//             .hash(state)
//     }
// }

// impl Deref for Vf32 {
//     type Target = Vec<f32>;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl From<Vec<f32>> for Vf32 {
//     fn from(value: Vec<f32>) -> Self {
//         Vf32(value)
//     }
// }

// #[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
// pub struct Vf64(pub Vec<f64>);

// impl Hash for Vf64 {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.0
//             .iter()
//             .map(|x| x.to_bits())
//             .collect::<Vec<u64>>()
//             .hash(state)
//     }
// }

// impl Deref for Vf64 {
//     type Target = Vec<f64>;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
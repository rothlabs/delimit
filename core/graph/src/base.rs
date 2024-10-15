use super::*;

pub trait Digest {
    fn digest<H: Hasher>(&self, state: &mut H);
}

macro_rules! hash_base {
    ($ty:ty) => {
        impl Digest for $ty {
            fn digest<H: Hasher>(&self, state: &mut H) {
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
hash_base!(i8);
hash_base!(i16);
hash_base!(i32);
hash_base!(i64);

impl Digest for f32 {
    fn digest<H: Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.to_bits(), state);
    }
}

impl Digest for f64 {
    fn digest<H: Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.to_bits(), state);
    }
}

impl<T: bytemuck::Pod> Digest for Vec<T> {
    fn digest<H: Hasher>(&self, state: &mut H) {
        let slice: &[u8] = bytemuck::cast_slice(self);
        std::hash::Hash::hash(slice, state);
    }
}

#[derive(Default, Debug, Clone)]
pub struct Buffer {
    id: u64,
    inner: Option<Grc<wgpu::Buffer>>,
}

impl Buffer {
    pub fn inner(&self) -> Result<&wgpu::Buffer> {
        if let Some(inner) = &self.inner {
            Ok(inner)
        } else {
            Err(anyhow!("no inner graph buffer"))?
        }
    }
}

impl Digest for Buffer {
    fn digest<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Serialize for Buffer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_unit()
    }
}

impl From<Grc<wgpu::Buffer>> for Buffer {
    fn from(value: Grc<wgpu::Buffer>) -> Self {
        Self {
            id: rand::random(),
            inner: Some(value)
        }
    }
}


// impl Deref for Buffer {
//     type Target = wgpu::Buffer;
//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }






// pub trait CastSlice {
//     fn slice<B>(&self) -> &[B]
//     where
//         B: bytemuck::AnyBitPattern;
// }

// impl CastSlice for Vec<u32> {
//     fn slice<B>(&self) -> &[B]
//     where
//         B: bytemuck::AnyBitPattern,
//     {
//         bytemuck::cast_slice(self)
//     }
// }

// impl CastSlice for Vec<f32> {
//     fn slice<B>(&self) -> &[B]
//     where
//         B: bytemuck::AnyBitPattern,
//     {
//         bytemuck::cast_slice(self)
//     }
// }

// impl Digest for Vec<f32> {
//     fn digest<H: Hasher>(&self, state: &mut H) {
//         let slice: &[u8] = bytemuck::cast_slice(self);
//         std::hash::Hash::hash(slice, state);
//     }
// }

// impl Digest for Vec<f64> {
//     fn digest<H: Hasher>(&self, state: &mut H) {
//         let slice: &[u8] = bytemuck::cast_slice(self);
//         std::hash::Hash::hash(slice, state);
//     }
// }

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

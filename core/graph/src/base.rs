use super::*;

pub trait Digest {
    fn digest<H: Hasher>(&self, state: &mut H);
}

impl<T: Digest> Digest for Vec<T> {
    fn digest<H: Hasher>(&self, state: &mut H) {
        for item in self {
            item.digest(state);
        }
    }
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

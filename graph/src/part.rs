use super::*;

pub enum Part {
    String(Ace<String>),
    F32(Ace<f32>),
    U8(Ace<u8>),
}

use super::*;

#[derive(Debug)]
pub enum Array {
    Vu16(Hub<u16>),
    Vf32(Hub<Vf32>),
}

impl Backed for Array {
    fn backed(&self, back: &Back) -> Self {
        
    }
}
use super::*;

// TODO: Make "Array" apex types to work in scenarios where different array types are valide like in the GPU


#[derive(Debug)]
pub enum Array {
    Vu16(Hub<u16>),
    Vf32(Hub<Vf32>),
}

impl Backed for Array {
    fn backed(&self, back: &Back) -> Self {
        
    }
}
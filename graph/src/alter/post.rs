use super::*;

pub struct Post {
    pub back: Back,
}

impl Backed for Post {
    fn backed(&self, back: &Back) -> Self {
        Self { back: back.clone() }
    }
}

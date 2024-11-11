use super::*;

pub struct Render<'a> {
    pub core: &'a Core,
    pub chain: Vec<Command>,
}

impl<'a> Render<'a> {
    pub fn direct(&self) {
        
    }
}
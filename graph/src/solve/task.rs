use super::*;

pub enum Task<'a> {
    None,
    Node,
    React,
    Serial(&'a mut Serial),
}

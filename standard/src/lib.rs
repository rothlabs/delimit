use std::collections::HashMap;

use graph::*;
use serde::Serialize;
use text::*;

#[derive(Serialize)]
pub struct Standard {
    pub links: HashMap<Meta, Role>,
}

/// Similar to graph role
#[derive(Serialize)]
pub struct Role {
    form: Form,
    part: Part,
}

#[derive(Serialize)]
pub enum Form {
    Text(Asset<String>),
    Array(Array),
}

#[derive(Serialize)]
pub enum Array {
    U8(Asset<Vec<u8>>),
    F32(Asset<Vec<f32>>),
}

#[derive(Serialize)]
pub enum Part {
    Text(plain::Part),
}



// pub enum Part {
//     Text(plain::Role),
//     Array(Array),
// }

// pub enum Array {
//     U8(Ace<Vec<u8>>),
//     F32(Ace<Vec<f32>>),
// }
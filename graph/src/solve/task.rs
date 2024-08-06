// use super::*;

pub enum Task {
    None,
    Main,
    Stems,
    React,
    Export,
    Find(String),
}

// pub enum Task<'a> {
//     None,
//     Node,
//     React,
//     Serial(&'a mut Serial),
// }

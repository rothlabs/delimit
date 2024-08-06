// use super::*;

pub enum Task {
    None,
    Main,
    Stems,
    React,
    Find(String),
    Cmd(String),
}

// pub enum Task<'a> {
//     None,
//     Node,
//     React,
//     Serial(&'a mut Serial),
// }

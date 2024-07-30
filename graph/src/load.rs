/// The Load enum allows Nodes to be handled with unknown payload datatype.
#[derive(Clone, PartialEq, Debug)]
pub enum Load {
    String(String),
    U8(u8),
    F32(f32),
    Vu8(Vec<u8>),
    Vf32(Vec<f32>),
    None,
}

// impl Default for Load {
//     fn default() -> Self {
//         Self::String("".to_owned())
//     }
// }
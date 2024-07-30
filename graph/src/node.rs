use super::*;
use std::error::Error;

pub type Result = std::result::Result<Node, Box<dyn Error + Send + Sync>>;

/// Contains a bare load, meta about a link, or the link itself.
#[derive(Clone, Default, PartialEq)]
pub struct Node {
    rank: usize,
    form: Form,
}

impl Node {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Node {
    pub fn field(&self, name: String) -> Field {
        Field::new(self.clone(), name)
    }
    pub fn meta(&self) -> Meta {
        self.form.meta()
    }
    pub fn insert(&self, field: &str, node: Node) {
        self.form.insert(field, node);
    }
    pub fn read_string<T, F: FnOnce(&String) -> T>(&self, read: F) -> T {
        self.form.read(|load|{
            if let Load::String(string) = load {
                read(string)
            } else {
                read(&"".into())
            }
        })
    }
    pub fn read_vu8<T, F: FnOnce(&Vec<u8>) -> T>(&self, read: F) -> T  {
        self.form.read(|load|{
            if let Load::Vu8(value) = load {
                read(value)
            } else {
                read(&vec![])
            }
        })
    }
    pub fn read_vu16<T, F: FnOnce(&Vec<u16>) -> T>(&self, read: F) -> T  {
        self.form.read(|load|{
            if let Load::Vu16(value) = load {
                read(value)
            } else {
                read(&vec![])
            }
        })
    }
    pub fn read_vf32<T, F: FnOnce(&Vec<f32>) -> T>(&self, read: F) -> T  {
        self.form.read(|load|{
            if let Load::Vf32(value) = load {
                read(value)
            } else {
                read(&vec![])
            }
        })
    }
    pub fn u32(&self) -> u32  {
        if let Load::U32(value) = self.load() {
            value
        } else {
            0
        }
    }
    pub fn i32(&self) -> i32  {
        if let Load::I32(value) = self.load() {
            value
        } else {
            0
        }
    }
}

impl ToLoad for Node {
    type Load = Load;
    fn load(&self) -> Self::Load {
        self.form.load()
    }
}

impl Read for Node {
    type Item = Load;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
        self.form.read(read)
    }
}

impl Grant for Node {
    type Load = Self;
    fn grant(&self) -> Self::Load {
        Self {
            rank: self.rank - 1,
            form: self.form.grant(),
        }
    }
}

impl Backed for Node {
    fn backed(&self, back: &Back) -> Self {
        Self {
            rank: self.rank,
            form: self.form.backed(back),
        }
    }
}

pub trait RankDown {
    /// Reduce node rank down to specified number.
    fn rank(&self, rank: usize) -> Self;
}

impl RankDown for Node {
    fn rank(&self, rank: usize) -> Self {
        let mut value = self.clone();
        while value.rank > rank {
            value = value.grant();
        }
        value
    }
}

impl RankDown for Vec<Node> {
    fn rank(&self, rank: usize) -> Self {
        self.iter().map(|x| x.rank(rank)).collect()
    }
}

/// Contains a bare load, meta about a link, or the link itself.
#[derive(Clone, PartialEq)]
pub enum Form {
    Meta(Meta),
    Bare(Load),
    Ace(Ace<Load>),
    Ploy(Ploy<Node>),
}

impl Form {
    fn meta(&self) -> Meta {
        match self {
            Self::Meta(meta) => meta.clone(),
            Self::Bare(_) => Meta::none(),
            Self::Ace(ace) => ace.meta(),
            Self::Ploy(ploy) => ploy.meta(),
        }
    }
    fn insert(&self, field: &str, node: Node) {
        match self {
            // Self::Ace(ace) => ace.insert(field, node),
            // Self::Ploy(ploy) => ploy.insert(field, node),
            _ => ()
        }
    }
}

impl Default for Form {
    fn default() -> Self {
        Self::Bare(Load::None)
    }
}

impl ToLoad for Form {
    type Load = Load;
    // TODO: load should take a link with repo traits
    fn load(&self) -> Self::Load {
        match self {
            // TODO: should attempt to lookup from repo
            Self::Meta(_) => Load::None,
            Self::Bare(bare) => bare.clone(),
            Self::Ace(ace) => ace.load(),
            Self::Ploy(ploy) => ploy.grant().load(),
        }
    }
}

impl Read for Form {
    type Item = Load;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
        match self {
            Self::Meta(_) => read(&Load::None),
            Self::Bare(bare) => read(bare),
            Self::Ace(ace) => ace.read(read),
            Self::Ploy(ploy) => ploy.grant().read(read),
        }
    }
}

impl Grant for Form {
    type Load = Self;
    fn grant(&self) -> Self::Load {
        match self {
            Self::Meta(_) => panic!("wrong level variant: meta"),
            Self::Bare(_) => panic!("wrong level variant: bare"),
            Self::Ace(_) => panic!("wrong level variant: ace"),
            Self::Ploy(ploy) => ploy.grant().form,
        }
    }
}

impl Backed for Form
// where
//     L: Clone,
{
    fn backed(&self, back: &Back) -> Self {
        match self {
            Self::Meta(meta) => Self::Meta(meta.clone()),
            Self::Bare(bare) => Self::Bare(bare.clone()),
            Self::Ace(ace) => Self::Ace(ace.backed(back)),
            Self::Ploy(ploy) => Self::Ploy(ploy.backed(back)),
        }
    }
}

impl From<Load> for Node {
    fn from(value: Load) -> Self {
        Self {
            rank: 0,
            form: Form::Bare(value),
        }
    }
}

impl From<Ace<Load>> for Node {
    fn from(value: Ace<Load>) -> Self {
        Self {
            rank: 0,
            form: Form::Ace(value),
        }
    }
}

impl From<Ploy<Node>> for Node
// where
//     L: 'static + Default,
{
    fn from(value: Ploy<Node>) -> Self {
        Self {
            rank: value.grant().rank + 1,
            form: Form::Ploy(value),
        }
    }
}

// impl<L> From<&L> for Value<L>
// where
//     L: Clone,
// {
//     fn from(value: &L) -> Self {
//         Self::Bare(value.clone())
//     }
// }

impl From<&Ace<Load>> for Node {
    fn from(value: &Ace<Load>) -> Self {
        Self {
            rank: 0,
            form: Form::Ace(value.clone()),
        }
    }
}

// impl<L> From<Ploy<L>> for Node<L> {
//     fn from(value: Ploy<L>) -> Self {
//         Self {
//             rank: 0,
//             form: Form::Bare(value.to_owned()),
//         }
//         Self::Ploy(value.clone())
//     }
// }

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        Self {
            rank: 0,
            form: Form::Bare(Load::String(value.to_owned())),
        }
    }
}

impl From<&Node> for Node {
    fn from(value: &Node) -> Self {
        value.clone()
    }
}



// pub fn read_string<T, F: FnOnce(&String) -> T>(&self, read: F) -> std::result::Result<T, T> {
//     self.form.read(|load|{
//         if let Load::String(string) = load {
//             Ok(read(string))
//         } else {
//             //panic!("not a string");
//             //Err("no string".to_owned())
//             Err(read(&"".into()))
//         }
//     })
// }










// // impl<L> From<Ploy<Ploy<Ace<L>>>> for Value<L>
// // where
// //     L: 'static + SendSync,
// // {
// //     fn from(value: Ploy<Ploy<Ace<L>>>) -> Self {
// //         Self::Ploy(Pipe::new(value).ploy())
// //     }
// // }

// // impl<L> From<&Ploy<Ploy<Ace<L>>>> for Value<L>
// // where
// //     L: 'static + SendSync,
// // {
// //     fn from(value: &Ploy<Ploy<Ace<L>>>) -> Self {
// //         Self::Ploy(Pipe::new(value.clone()).ploy())
// //     }
// // }

// // impl<L> From<&Vec<Value<L>>> for Value<L>
// // where
// //     L: Clone
// // {
// //     fn from(value: &Vec<Value<L>>) -> Self {
// //         value.clone()
// //     }
// // }

// // impl From<&str> for &Value<String> {
// //     fn from(value: &str) -> Self {
// //         Self::Bare(value.to_owned())
// //     }
// // }

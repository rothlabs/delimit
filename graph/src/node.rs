use super::*;
use std::result;

pub type Result = result::Result<Node, Error>;

/// Graph node. The Form could be Meta, Load, Leaf, or Ploy.
#[derive(Clone, Default, PartialEq)]
pub struct Node {
    rank: usize,
    form: Form,
}

impl Node {
    pub fn empty() -> solve::Result {
        Ok(Tray::Node(Self::default()))
    }
    pub fn load(&self) -> load::Result {
        self.form.load()
    }
    /// Solve the node for the next node until the given rank.
    pub fn at(&self, rank: usize) -> Result {
        let mut node = self.clone();
        while node.rank > rank {
            node = node.query().node()?;
        }
        Ok(node)
    }
    pub fn field(&self, name: String) -> Field {
        Field::new(self.clone(), name)
    }
    pub fn meta(&self) -> Meta {
        self.form.meta()
    }
    pub fn read_or_error<T, F: FnOnce(&Load) -> T>(&self, read: F) -> result::Result<T, Error> {
        self.form.read(|load| match load {
            Ok(value) => Ok(read(value)),
            _ => Err("nothing to read".into()),
        })
    }
    pub fn read<T, F: FnOnce(load::ResultRef) -> T>(&self, read: F) -> T {
        self.form.read(read)
    }
    pub fn read_string<T, F: FnOnce(&String) -> T>(&self, read: F) -> T {
        self.form.read(|load| match load {
            Ok(Load::String(value)) => read(value),
            _ => read(&"".into()),
        })
    }
    pub fn read_vu8<T, F: FnOnce(&Vec<u8>) -> T>(&self, read: F) -> T {
        self.form.read(|load| match load {
            Ok(Load::Vu8(value)) => read(value),
            _ => read(&vec![]),
        })
    }
    pub fn read_vu16<T, F: FnOnce(&Vec<u16>) -> T>(&self, read: F) -> T {
        self.form.read(|load| match load {
            Ok(Load::Vu16(value)) => read(value),
            _ => read(&vec![]),
        })
    }
    pub fn read_vf32<T, F: FnOnce(&Vec<f32>) -> T>(&self, read: F) -> T {
        self.form.read(|load| match load {
            Ok(Load::Vf32(value)) => read(value),
            _ => read(&vec![]),
        })
    }
    pub fn u32(&self) -> u32 {
        match self.load() {
            Ok(Load::U32(value)) => value,
            _ => 0,
        }
    }
    pub fn i32(&self) -> i32 {
        match self.load() {
            Ok(Load::I32(value)) => value,
            _ => 0,
        }
    }
}

impl Solve for Node {
    fn solve(&self, task: Task) -> solve::Result {
        Ok(Self {
            rank: self.rank - 1,
            form: self.form.solve(task)?,
        }
        .into())
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
    fn at(&self, rank: usize) -> result::Result<Vec<Node>, Error>;
}

impl RankDown for Vec<Node> {
    fn at(&self, rank: usize) -> result::Result<Vec<Node>, Error> {
        self.iter().map(|x| x.at(rank)).collect()
    }
}

/// Contains a bare load, meta about a link, or the link itself.
#[derive(Clone, PartialEq)]
pub enum Form {
    Meta(Meta),
    Bare(Load),
    Leaf(Leaf),
    Ploy(Ploy),
}

impl Form {
    // TODO: make fallible
    fn meta(&self) -> Meta {
        match self {
            Self::Meta(meta) => meta.clone(),
            Self::Bare(_) => Meta::none(),
            Self::Leaf(leaf) => leaf.meta(),
            Self::Ploy(ploy) => ploy.meta(),
        }
    }
    fn load(&self) -> load::Result {
        match self {
            // TODO: should attempt to lookup from repo before error
            Self::Meta(_) => Err("not a load".into()),
            Self::Bare(bare) => Ok(bare.clone()),
            Self::Leaf(leaf) => Ok(leaf.load()),
            Self::Ploy(ploy) => ploy.query().node()?.load(),
        }
    }
    fn read<T, F: FnOnce(load::ResultRef) -> T>(&self, read: F) -> T {
        match self {
            Self::Meta(_) => read(Err("nothing to read".into())),
            Self::Bare(bare) => read(Ok(bare)),
            Self::Leaf(leaf) => leaf.read_load(read),
            Self::Ploy(ploy) => {
                if let Ok(node) = ploy.query().node() {
                    node.read(read)
                } else {
                    read(Err("failed to read ploy".into()))
                }
            }
        }
    }
    fn solve(&self, _: Task) -> result::Result<Form, Error> {
        match self {
            Self::Meta(_) => Err("not a ploy".into()),
            Self::Bare(_) => Err("not a ploy".into()),
            Self::Leaf(_) => Err("not a ploy".into()),
            Self::Ploy(ploy) => Ok(ploy.query().node()?.form),
        }
    }
}

impl Default for Form {
    fn default() -> Self {
        Self::Bare(Load::None)
    }
}

impl Backed for Form {
    fn backed(&self, back: &Back) -> Self {
        match self {
            Self::Meta(meta) => Self::Meta(meta.clone()),
            Self::Bare(bare) => Self::Bare(bare.clone()),
            Self::Leaf(leaf) => Self::Leaf(leaf.backed(back)),
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

impl From<Leaf> for Node {
    fn from(leaf: Leaf) -> Self {
        Self {
            rank: 0,
            form: Form::Leaf(leaf),
        }
    }
}

impl From<Ploy> for Node {
    fn from(ploy: Ploy) -> Self {
        // TODO: find way to not query the node to get rank!
        let rank = match ploy.query().node() {
            Ok(node) => node.rank + 1,
            _ => 0,
        };
        Self {
            rank,
            form: Form::Ploy(ploy),
        }
    }
}

impl From<&Leaf> for Node {
    fn from(value: &Leaf) -> Self {
        Self {
            rank: 0,
            form: Form::Leaf(value.clone()),
        }
    }
}

impl From<&Node> for Node {
    fn from(value: &Node) -> Self {
        value.clone()
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        Self {
            rank: 0,
            form: Form::Bare(Load::String(value.to_owned())),
        }
    }
}

impl From<u32> for Node {
    fn from(value: u32) -> Self {
        Self {
            rank: 0,
            form: Form::Bare(Load::U32(value)),
        }
    }
}

impl From<i32> for Node {
    fn from(value: i32) -> Self {
        Self {
            rank: 0,
            form: Form::Bare(Load::I32(value)),
        }
    }
}

impl From<Vec<u8>> for Node {
    fn from(value: Vec<u8>) -> Self {
        Self {
            rank: 0,
            form: Form::Leaf(Leaf::new(Load::Vu8(value))),
        }
    }
}

impl From<Vec<u16>> for Node {
    fn from(value: Vec<u16>) -> Self {
        Self {
            rank: 0,
            form: Form::Leaf(Leaf::new(Load::Vu16(value))),
        }
    }
}

impl From<Vec<f32>> for Node {
    fn from(value: Vec<f32>) -> Self {
        Self {
            rank: 0,
            form: Form::Leaf(Leaf::new(Load::Vf32(value))),
        }
    }
}

// impl ToLoad for Form {
//     type Load = Load;
//     // TODO: load should take a link with repo traits
//     fn load(&self) -> Self::Load {
//         match self {
//             // TODO: should attempt to lookup from repo
//             Self::Meta(_) => Load::None,
//             Self::Bare(bare) => bare.clone(),
//             Self::Ace(leaf) => leaf.load(),
//             Self::Ploy(ploy) => {
//                 let wow = ploy.query().node()?;
//                 ploy.solve().load()
//             }
//         }
//     }
// }

// impl From<Vec<u8>> for Node {
//     fn from(value: Vec<u8>) -> Self {
//         Self {
//             rank: 0,
//             form: Form::Bare(Load::Vu8(value)),
//         }
//     }
// }

// impl<L> From<&L> for Value<L>
// where
//     L: Clone,
// {
//     fn from(value: &L) -> Self {
//         Self::Bare(value.clone())
//     }
// }

// impl<L> From<Ploy<L>> for Node<L> {
//     fn from(value: Ploy<L>) -> Self {
//         Self {
//             rank: 0,
//             form: Form::Bare(value.to_owned()),
//         }
//         Self::Ploy(value.clone())
//     }
// }

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

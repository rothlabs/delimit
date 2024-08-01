use super::*;
use std::result;

pub type Result = result::Result<Node, Error>;

/// Contains a bare load, meta about a link, or the link itself.
#[derive(Clone, Default, PartialEq)]
pub struct Node {
    rank: usize,
    form: Form,
}

impl Node {
    // pub fn new() -> Self {
    //     Self::default()
    // }
    pub fn empty() -> solve::Result {
        Ok(Tray::Node(Self::default()))
    }
    // pub fn query(&self) -> Query<Self> {
    //     Query::new(self)
    // }
    pub fn load(&self) -> load::Result {
        self.form.load()
    }
    pub fn rank(&self, rank: usize) -> Result {
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
    pub fn insert(&self, field: &str, node: Node) {
        self.form.insert(field, node);
    }
    fn read<T, F: FnOnce(load::ResultRef) -> T>(&self, read: F) -> T {
        self.form.read(read)
    }
    pub fn read_string<T, F: FnOnce(&String) -> T>(&self, read: F) -> T {
        self.form.read(|load|{
            if let Ok(load) = load {
                if let Load::String(string) = load {
                    return read(string)
                }
            }
            read(&"".into())
        })
    }
    pub fn read_vu8<T, F: FnOnce(&Vec<u8>) -> T>(&self, read: F) -> T  {
        self.form.read(|load|{
            if let Ok(load) = load {
                if let Load::Vu8(value) = load {
                    return read(value)
                }
            }
            read(&vec![])
        })
    }
    pub fn read_vu16<T, F: FnOnce(&Vec<u16>) -> T>(&self, read: F) -> T  {
        self.form.read(|load|{
            if let Ok(load) = load {
                if let Load::Vu16(value) = load {
                    return read(value)
                }
            }
            read(&vec![])
        })
    }
    pub fn read_vf32<T, F: FnOnce(&Vec<f32>) -> T>(&self, read: F) -> T  {
        self.form.read(|load|{
            if let Ok(load) = load {
                if let Load::Vf32(value) = load {
                    return read(value)
                }
            }
            read(&vec![])
        })
    }
    pub fn u32(&self) -> u32  {
        if let Ok(load) = self.load() {
            if let Load::U32(value) = load {
                return value
            }
        }
        0
    }
    pub fn i32(&self) -> i32  {
        if let Ok(load) = self.load() {
            if let Load::I32(value) = load {
                return value
            }
        }
        0
    }
}

// impl ToLoad for Node {
//     type Load = Load;
//     fn load(&self) -> Self::Load {
//         self.form.load()
//     }
// }

// impl Read for Node {
//     type Item = load::Result;
//     fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
//         self.form.read(read)
//     }
// }

impl Solve for Node {
    fn solve(&self) -> solve::Result {
        Ok(Self {
            rank: self.rank - 1,
            form: self.form.solve()?,
        }.into())
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
    fn rank(&self, rank: usize) -> result::Result<Vec<Node>, Error>;
}

impl RankDown for Vec<Node> {
    fn rank(&self, rank: usize) -> result::Result<Vec<Node>, Error> {
        self.iter().map(|x| Ok(x.rank(rank)?)).collect()
    }
}

/// Contains a bare load, meta about a link, or the link itself.
#[derive(Clone, PartialEq)]
pub enum Form {
    Meta(Meta),
    Bare(Load),
    Ace(Ace),
    Ploy(Ploy),
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
    fn load(&self) -> load::Result {
        match self {
            // TODO: should attempt to lookup from repo before error
            Self::Meta(_) => Err("not a load".into()),
            Self::Bare(bare) => Ok(bare.clone()),
            Self::Ace(ace) => Ok(ace.load()),
            Self::Ploy(ploy) => ploy.query().node()?.load(),
        }
    }
    fn read<T, F: FnOnce(load::ResultRef) -> T>(&self, read: F) -> T {
        match self {
            Self::Meta(_) => read(Err("not a load".into())),
            Self::Bare(bare) => read(Ok(bare)),
            Self::Ace(ace) => ace.read_load(read),
            Self::Ploy(ploy) => {
                if let Ok(node) = ploy.query().node() {
                    node.read(read)
                } else {
                    read(Err("not a load".into()))
                }
            },
        }
    }
    fn solve(&self) -> result::Result<Form, Error> {
        match self {
            Self::Meta(_) => Err("not a solver".into()),
            Self::Bare(_) => Err("not a solver".into()),
            Self::Ace(_) => Err("not a solver".into()),
            Self::Ploy(ploy) => Ok(ploy.query().node()?.form),
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

// impl Read for Form {
//     type Item = Load;
//     fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
//         match self {
//             Self::Meta(_) => read(&Load::None),
//             Self::Bare(bare) => read(bare),
//             Self::Ace(ace) => ace.read(read),
//             Self::Ploy(ploy) => ploy.query().node()?.read(read),
//         }
//     }
// }

// impl Form {
//     fn solve(&self) -> Self {
//         match self {
//             Self::Meta(_) => panic!("wrong level variant: meta"),
//             Self::Bare(_) => panic!("wrong level variant: bare"),
//             Self::Ace(_) => panic!("wrong level variant: ace"),
//             Self::Ploy(ploy) => ploy.solve().form,
//         }
//     }
// }

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

impl From<Ace> for Node {
    fn from(ace: Ace) -> Self {
        Self {
            rank: 0,
            form: Form::Ace(ace),
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

impl From<&Ace> for Node {
    fn from(value: &Ace) -> Self {
        Self {
            rank: 0,
            form: Form::Ace(value.clone()),
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
            form: Form::Bare(Load::Vu8(value)),
        }
    }
}

impl From<Vec<u16>> for Node {
    fn from(value: Vec<u16>) -> Self {
        Self {
            rank: 0,
            form: Form::Bare(Load::Vu16(value)),
        }
    }
}

impl From<Vec<f32>> for Node {
    fn from(value: Vec<f32>) -> Self {
        Self {
            rank: 0,
            form: Form::Bare(Load::Vf32(value)),
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
//             Self::Ace(ace) => ace.load(),
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

use super::*;

/// Contains a bare load, meta about a link, or the link itself.
#[derive(Clone, PartialEq, Serialize)]
pub enum Form {
    Meta(Meta),
    Load(Load),
    Leaf(Leaf),
    Ploy(Ploy),
}

impl Form {
    // TODO: make fallible
    pub fn meta(&self) -> Meta {
        match self {
            Self::Meta(meta) => meta.clone(),
            Self::Load(_) => Meta::none(),
            Self::Leaf(leaf) => leaf.meta(),
            Self::Ploy(ploy) => ploy.meta(),
        }
    }
    pub fn serial(&self, serial: &mut Serial) -> serial::Result {
        match self {
            Self::Meta(_) => Ok(()),
            Self::Load(_) => Ok(()),
            Self::Leaf(leaf) => leaf.serial(serial),
            Self::Ploy(ploy) => ploy.serial(serial),
        }
    }
    pub fn load(&self) -> load::Result {
        match self {
            // TODO: should attempt to lookup from repo before error
            Self::Meta(_) => Err("not a load".into()),
            Self::Load(bare) => Ok(bare.clone()),
            Self::Leaf(leaf) => Ok(leaf.load()),
            Self::Ploy(ploy) => ploy.query().main()?.load(),
        }
    }
    pub fn read<T, F: FnOnce(load::ResultRef) -> T>(&self, read: F) -> T {
        match self {
            Self::Meta(_) => read(Err("nothing to read".into())),
            Self::Load(bare) => read(Ok(bare)),
            Self::Leaf(leaf) => leaf.read_load(read),
            Self::Ploy(ploy) => {
                if let Ok(node) = ploy.query().main() {
                    node.read(read)
                } else {
                    read(Err("failed to read ploy".into()))
                }
            }
        }
    }
    pub fn solve_form(&self, _: Task) -> result::Result<Form, Error> {
        match self {
            Self::Meta(_) => Err("not a ploy".into()),
            Self::Load(_) => Err("not a ploy".into()),
            Self::Leaf(_) => Err("not a ploy".into()),
            Self::Ploy(ploy) => Ok(ploy.query().main()?.form),
        }
    }
    pub fn solve(&self, task: Task) -> solve::Result {
        match self {
            Self::Meta(_) => Err("not a ploy".into()),
            Self::Load(_) => Err("not a ploy".into()),
            Self::Leaf(_) => Err("not a ploy".into()),
            Self::Ploy(ploy) => ploy.solve(task),
        }
    }
    pub fn alter(&self, post: Post) -> alter::Result {
        match self {
            Self::Meta(_) => Err("not a ploy".into()),
            Self::Load(_) => Err("not a ploy".into()),
            Self::Leaf(_) => Err("not a ploy".into()),
            Self::Ploy(ploy) => ploy.alter(post),
        }
    }
}

impl Default for Form {
    fn default() -> Self {
        Self::Load(Load::None)
    }
}

// impl Serialize for Form {
//     fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
//         match self {
//             Self::Meta(meta) => meta.serialize(serializer),
//             Self::Bare(load) => load.serialize(serializer),
//             Self::Leaf(leaf) => leaf.serialize(serializer),
//             Self::Ploy(ploy) => ploy.serialize(serializer),

//         }
//     }
// }

impl Backed for Form {
    fn backed(&self, back: &Back) -> Self {
        match self {
            Self::Meta(meta) => Self::Meta(meta.clone()),
            Self::Load(bare) => Self::Load(bare.clone()),
            Self::Leaf(leaf) => Self::Leaf(leaf.backed(back)),
            Self::Ploy(ploy) => Self::Ploy(ploy.backed(back)),
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

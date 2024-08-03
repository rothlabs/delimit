pub use form::Form;

use super::*;
use std::result;

mod form;

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
    /// Solve the node for the next node down until the given rank.
    pub fn at(&self, rank: usize) -> Result {
        let mut node = self.clone();
        while node.rank > rank {
            node = node.query().node()?;
        }
        Ok(node)
    }
    pub fn meta(&self) -> Meta {
        self.form.meta()
    }
    pub fn serial(&self, serial: &mut Serial) -> serial::Result {
        if !serial.contains(&self.meta()) {
            return self.form.serial(serial)
        }
        Ok(())
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

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.form.serialize(serializer)
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

impl Alter for Node {
    fn alter(&mut self, post: Post) -> alter::Result {
        self.form.alter(post)
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

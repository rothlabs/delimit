pub use form::Form;

use super::*;
use std::result;

mod form;
// mod alter;

pub type Result = result::Result<Node, Error>;

/// Graph node. The Form could be Meta, Load, Leaf, or Ploy.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Node {
    /// TODO: rank should go in Meta! Form can be Node!
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
    /// Trade for another node via base.
    pub fn trade(&self, base: &dyn Trade) -> Self {
        base.trade(self)
    }
    /// Solve down to the given rank.
    pub fn at(&self, rank: usize) -> Result {
        let mut node = self.clone();
        while node.rank > rank {
            node = node.query().main()?;
        }
        Ok(node)
    }
    pub fn meta(&self) -> Meta {
        self.form.meta()
    }
    pub fn serial(&self, serial: &mut Serial) -> serial::Result {
        if !serial.contains(&self.meta()) {
            return self.form.serial(serial);
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
    pub fn string(&self) -> result::Result<String, Error> {
        match self.load() {
            Ok(Load::String(value)) => Ok(value),
            _ => Err("not a string".into()),
        }
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

pub trait TradeNode {
    /// Trade nodes for others via base.
    fn trade(&self, base: &dyn Trade) -> Self;
}

impl TradeNode for Vec<Node> {
    fn trade(&self, base: &dyn Trade) -> Self {
        self.iter().map(|x| x.trade(base)).collect()
    }
}

impl Solve for Node {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => Ok(Self {
                rank: self.rank - 1,
                form: self.form.solve_form(task)?,
            }
            .into()),
            _ => self.form.solve(task),
        }
    }
}

impl AdaptInner for Node {
    fn adapt(&self, post: Post) -> adapt::Result {
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

pub trait SolveDown {
    /// Reduce node rank down to specified number.
    fn at(&self, rank: usize) -> result::Result<Vec<Node>, Error>;
}

impl SolveDown for Vec<Node> {
    fn at(&self, rank: usize) -> result::Result<Vec<Node>, Error> {
        self.iter().map(|x| x.at(rank)).collect()
    }
}

impl From<Load> for Node {
    fn from(value: Load) -> Self {
        Self {
            rank: 0,
            form: Form::Load(value),
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
        let rank = match ploy.query().main() {
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
            form: Form::Load(Load::String(value.to_owned())),
        }
    }
}

impl From<u32> for Node {
    fn from(value: u32) -> Self {
        Self {
            rank: 0,
            form: Form::Load(Load::U32(value)),
        }
    }
}

impl From<i32> for Node {
    fn from(value: i32) -> Self {
        Self {
            rank: 0,
            form: Form::Load(Load::I32(value)),
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

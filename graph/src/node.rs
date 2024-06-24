mod leaf;
mod solver;

pub use leaf::Leaf;
pub use solver::Solver;

pub trait Read {
    type Unit;
    fn read(&self) -> &Self::Unit;
}

pub trait Write {
    type Unit;
    fn write<F: FnOnce(&mut Self::Unit)>(&mut self, write: F); //  -> &mut Self::Unit
}

pub trait Respond {
    type Memo;
    fn respond(&mut self, memo: Self::Memo);
}
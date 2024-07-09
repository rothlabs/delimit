pub mod edge;
pub mod link;
pub mod meta;
pub mod node;
pub mod react;
pub mod read;
pub mod repo;
pub mod unit;
pub mod view;
pub mod work;
pub mod write;

pub use link::{Link, Leaf, IntoLeaf, Solver, ToLeaf, UnitSolver};
pub use meta::Meta;
pub use react::{
    AddRoot, React, ReactMut, Reactor, Reactors, SolverWithReactor, TaskerWithReactor, ToReactor,
    WithRoot, Cycle,
};
pub use read::{Read, Reader, Solve, SolveTask};
pub use repo::Repo;
pub use unit::Gate;
pub use view::{AddStr, AddToLeafViews, LeafEye, LeafView, ToLeafViewsBuilder};
pub use work::{Work, UnitLoad};
pub use write::{
    SolveMut, SolveTaskMut, Write, WriteWithReactor, WriterWithReactor, Writer, Pack, WriterWithPack,
};
pub use node::Node;
pub use edge::Edge;

const NO_POISON: &str = "the lock should not be poisoned";

pub trait ToWork {
    type Work;
    fn work(&self) -> Self::Work;
}

pub trait SolveShare<L>: Solve<Load = L> + SolverWithReactor<Load = L> {}

pub trait SolveTaskShare<T, L>:
    SolveTask<Task = T, Load = L> + TaskerWithReactor<Task = T, Load = L>
{
}

pub trait ToSolver {
    type Load;
    fn solver(&self) -> Solver<Self::Load>;
}

// pub trait ToTasker {
//     type Task;
//     type Load;
//     fn tasker(&self) -> link::Tasker<Self::Task, Self::Load>;
// }

pub trait AddStem {
    type Unit;
    fn add_stem<T, F: FnOnce(&mut Self::Unit, T)>(&mut self, stem: T, add_stem: F);
}

pub trait Clear {
    fn clear(&mut self);
}

pub trait FromUnit {
    type Unit;
    fn from_unit(unit: Self::Unit) -> Self;
}

pub trait FromLoad {
    type Load;
    fn from_load(unit: Self::Load) -> Self;
}

pub trait Memory {
    type Task: Clone;
    type Load: Clone;
    fn add(&mut self, task: Self::Task, load: Self::Load);
    fn get(&self, task: &Self::Task) -> Option<&Self::Load>;
}

pub trait SolveWithReactor {
    //////////////////////////////////////////////////
    type Item;
    fn solve_with_reactor(&self, reactor: &Reactor) -> Self::Item;
}

pub struct Role<L, E> {
    pub exact: E,
    pub solver: Solver<L>,
}

impl<L, E> Clone for Role<L, E> 
where 
    E: Clone,
{
    fn clone(&self) -> Self {
        Self {
            exact: self.exact.clone(),
            solver: self.solver.clone(),
        }
    }
}

impl<L, E> Solve for Role<L, E> {
    type Load = L;
    fn solve(&self) -> Self::Load {
        self.solver.solve()
    }
}

impl<L, E> WithRoot for Role<L, E>
where
    E: Clone,
{
    type Root = Reactor;
    fn with_root(&self, root: &Self::Root) -> Self {
        Self {
            exact: self.exact.clone(),
            solver: self.solver.with_reactor(root),
        }
    }
}

// pub trait Stemmer {
//     type Unit;
//     fn stemmer<T: WithReactor, F: FnOnce(&mut Self::Unit, T)>(&self, stem: &T, add_stem: F);
// }

// impl<L: 'static> SolveLay<L> {
//     pub fn read<F: FnOnce(&L)>(&self, read: F) {
//         match self {
//             SolveLay::Bare(unit) => read(unit),
//             SolveLay::Leaf(leaf) => leaf.reader(read),
//             SolveLay::Solver(_) => panic!("cannot read link::Solver<U>"),
//         };
//     }
// }

// /// Make a string. ToLeaf comes for free.
// pub trait GraphString {
//     fn string(&self) -> String;
// }

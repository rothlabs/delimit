use crate::*;

//pub use leaf::Leaf;
//pub use unit_solver::UnitSolver;
//pub use unit_tasker::UnitTasker;

//mod leaf;
//mod unit_solver;
//mod unit_tasker;

pub type Leaf<L> = Node<Reactors, Bare<L>>;

pub type UnitSolver<U, L> = Node<Reactors, Pair<U, L>>;

pub struct Node<R, W> {
    root: R,
    work: W,
}

impl<R, W> ToLoad for Node<R, W> 
where 
    W: ToLoad,
{
    type Load = W::Load;
    fn load(&self) -> Self::Load {
        self.work.load()
    }
}

// impl<R, W> FromLoad for Node<R, W> 
// where 
//     R: Default
// {
//     type Load = W;
//     fn from_load(unit: Self::Load) -> Self {
//         Self {
//             root: R::default(),
//             work: unit, 
//         }
//     }
// }

impl<R, W> FromWorkItem for Node<R, W> 
where 
    R: Default,
    W: FromWorkItem,
{
    type Item = W::Item;
    fn new(unit: Self::Item) -> Self {
        Self { 
            root: R::default(),
            work: W::new(unit), 
        }
    }
}

impl<R, W> Write for Node<R, W> 
where 
    R: Cycle,
    W: Write,
{
    type Unit = W::Unit;
    fn write<F: FnOnce(&mut Self::Unit)>(&mut self, write: F) {
        self.work.write(write);
        self.root.cycle();
    }
}

impl<R, W> WriteWithReactor for Node<R, W> 
where 
    R: Cycle,
    W: WriteWithReactor,
{
    type Unit = W::Unit;
    fn write_with_reactor<F: FnOnce(&mut Pack<Self::Unit>)>(&mut self, write: F, reactor: &Reactor) {
        self.work.write_with_reactor(write, reactor);
        self.root.cycle();
    }
}

impl<R, W> Read for Node<R, W> 
where 
    W: Read,
{
    type Unit = W::Unit;
    fn read(&self) -> &Self::Unit {
        self.work.read()
    }
}

impl<R, W> SolveMut for Node<R, W>
where
    W: SolveMut,
{
    type Load = W::Load;
    fn solve_mut(&mut self) -> Self::Load {
        self.work.solve_mut()
    }
}

impl<R, W> ReactMut for Node<R, W> 
where 
    R: React,
    W: Clear,
{
    fn clear(&mut self) -> Reactors {
        self.work.clear();
        self.root.clear()
    }
    fn react(&mut self) {
        
    }
}

impl<R, W> AddRoot for Node<R, W> 
where 
    R: AddRoot,
{
    type Item = R::Item;
    fn add_root(&mut self, root: Self::Item) {
        self.root.add_root(root);
    }
}

// impl<U, L> SolveMut for UnitSolver<U, L>
// where
//     U: Solve<Load = L>,
//     L: Clone,
// {
//     type Load = L;
//     fn solve_mut(&mut self) -> Self::Load {
//         //self.work.solve()
//     }
// }

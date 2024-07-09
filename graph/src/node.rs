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

impl<R, W> FromItem for Node<R, W> 
where 
    R: Default,
    W: FromItem,
{
    type Item = W::Item;
    fn new(item: Self::Item) -> Self {
        Self { 
            root: R::default(),
            work: W::new(item), 
        }
    }
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
    fn write_with_reactor<F: FnOnce(&mut Pack<Self::Unit>)>(&mut self, write: F, reactor: &RootNode) {
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

impl<R, W> EventReactMut for Node<R, W> 
where 
    R: Event<Roots = Reactors>,
    W: Clear,
{}

impl<R, W> EventMut for Node<R, W> 
where 
    R: Event<Roots = Reactors>,
    W: Clear,
{
    type Roots = R::Roots;
    fn event_mut(&mut self) -> Self::Roots {
        self.work.clear();
        self.root.event()
    }
}

impl<R, W> ReactMut for Node<R, W> {
    fn react_mut(&mut self) {
        
    }
}

impl<R, W> AddRoot for Node<R, W> 
where 
    R: AddRoot,
{
    type Root = R::Root;
    fn add_root(&mut self, root: Self::Root) {
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

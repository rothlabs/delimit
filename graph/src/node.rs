use serde::Serialize;

use crate::*;

pub type Sole<L> = Node<Ring, work::Sole<L>>;
pub type Pair<U, L> = Node<Ring, work::Pair<U, L>>;

/// A node creates an interactive bridge between root edges and work.
#[derive(Serialize)]
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

impl<R, W> ToSerial for Node<R, W>
where
    W: Serialize,
{
    fn serial(&mut self, serial: &'static mut Serial) -> &mut Serial {
        // TODO: need to call serial on work as well and put items in HashMap with key as ID!!!
        serial.add(&self.work)
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

impl<R, W> WriteWithRoot for Node<R, W>
where
    R: Cycle,
    W: WriteWithRoot,
{
    type Unit = W::Unit;
    fn write_with_root<F: FnOnce(&mut Pack<Self::Unit>)>(&mut self, write: F, root: &Root) {
        self.work.write_with_root(write, root);
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

impl<R, W> AddRoot for Node<R, W>
where
    R: AddRoot,
{
    type Root = R::Root;
    fn add_root(&mut self, root: Self::Root) {
        self.root.add_root(root);
    }
}

impl<R, W> EventReactMut for Node<R, W>
where
    R: Event<Root = Ring>, // + Send + Sync,
    W: Clear,              // + Send + Sync,
{
}

impl<R, W> EventMut for Node<R, W>
where
    R: Event<Root = Ring>,
    W: Clear,
{
    type Roots = R::Root;
    fn event_mut(&mut self) -> Self::Roots {
        self.work.clear();
        self.root.event()
    }
}

impl<R, W> ReactMut for Node<R, W> {
    fn react_mut(&mut self) {}
}

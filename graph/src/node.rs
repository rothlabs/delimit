use serde::Serialize;

use crate::*;

pub type Sole<L> = Node<Ring, work::Sole<L>>;
pub type Pair<U, L> = Node<Ring, work::Pair<U, L>>;
pub type Trey<U, T, L> = Node<Ring, work::Trey<U, T, L>>;

/// A node creates an interactive bridge between root edges and work.
#[derive(Serialize)]
pub struct Node<R, W> {
    ring: R,
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
            ring: R::default(),
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
        self.ring.cycle();
    }
}

impl<R, W> WriteWithRoot for Node<R, W>
where
    R: Cycle,
    W: WriteWithRoot,
{
    type Unit = W::Unit;
    fn write_with_root<F: FnOnce(&mut Pack<Self::Unit>)>(&mut self, write: F, root: &Back) {
        self.work.write_with_root(write, root);
        self.ring.cycle();
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

impl<R, W> Grantor for Node<R, W>
where
    W: Grantor,
{
    type Load = W::Load;
    fn grantor(&mut self) -> Self::Load {
        self.work.grantor()
    }
}

impl<R, W> Solver for Node<R, W>
where
    W: Solver,
{
    type Task = W::Task;
    type Load = W::Load;
    fn solver(&mut self, task: Self::Task) -> Self::Load {
        self.work.solver(task)
    }
}

impl<R, W> AddRoot for Node<R, W>
where
    R: AddRoot,
{
    type Root = R::Root;
    fn add_root(&mut self, root: Self::Root) {
        self.ring.add_root(root);
    }
}

impl<R, W> Updater for Node<R, W>
where
    R: Rebut<Ring = Ring>,
    W: Clear,
{
}

impl<R, W> Rebuter for Node<R, W>
where
    R: Rebut<Ring = Ring>,
    W: Clear,
{
    type Ring = R::Ring;
    fn rebuter(&mut self) -> Self::Ring {
        self.work.clear();
        self.ring.rebut()
    }
}

impl<R, W> Reactor for Node<R, W> {
    fn reactor(&mut self) {}
}

use serde::Serialize;

use crate::*;

pub type Ace<L> = Node<work::Ace<L>>;
pub type Deuce<U, L> = Node<work::Deuce<U, L>>;
pub type Trey<U, T, L> = Node<work::Trey<U, T, L>>;

/// A node creates an interactive bridge between root edges and work.
pub struct Node<W> {
    meta: Meta,
    ring: Ring,
    work: W,
}

impl<W> FromItem for Node<W>
where
    W: FromItem,
{
    type Item = W::Item;
    fn new(item: Self::Item) -> Self {
        Self {
            meta: Meta::new(),
            ring: Ring::new(),
            work: W::new(item),
        }
    }
}

impl<W> ToSerial for Node<W>
where
    W: Serialize,
{
    fn serial(&mut self, serial: &'static mut Serial) -> &mut Serial {
        // TODO: need to call serial on work as well and put items in HashMap with key as ID!!!
        serial.add(&self.work)
    }
}

impl<W> ToLoad for Node<W>
where
    W: ToLoad,
{
    type Load = W::Load;
    fn load(&self) -> Self::Load {
        self.work.load()
    }
}

impl<W> Write for Node<W>
where
    W: Write,
{
    type Item = W::Item;
    fn write<F: FnOnce(&mut Self::Item)>(&mut self, write: F) {
        self.work.write(write);
        self.ring.cycle(&self.meta);
    }
}

impl<W> WriteWithBack for Node<W>
where
    W: WriteWithBack,
{
    type Unit = W::Unit;
    fn write_with_back<F: FnOnce(&mut Pack<Self::Unit>)>(&mut self, write: F, back: &Back) {
        self.work.write_with_back(write, back);
        self.ring.cycle(&self.meta);
    }
}

impl<W> Read for Node<W>
where
    W: Read,
{
    type Item = W::Item;
    fn read(&self) -> &Self::Item {
        self.work.read()
    }
}

impl<W> Grantor for Node<W>
where
    W: Grantor,
{
    type Load = W::Load;
    fn grantor(&mut self) -> Self::Load {
        self.work.grantor()
    }
}

impl<W> Solver for Node<W>
where
    W: Solver,
{
    type Task = W::Task;
    type Load = W::Load;
    fn solver(&mut self, task: Self::Task) -> Self::Load {
        self.work.solver(task)
    }
}

impl<W> AddRoot for Node<W> {
    fn add_root(&mut self, root: Root) {
        self.ring.add_root(root);
    }
}

impl<W> Update for Node<W> where W: Clear + React {}

impl<W> Rebut for Node<W>
where
    W: Clear,
{
    fn rebut(&mut self) -> Ring {
        self.work.clear();
        self.ring.rebut()
    }
}

impl<W> React for Node<W>
where
    W: React,
{
    fn react(&mut self, meta: &Meta) {
        self.work.react(meta);
    }
}

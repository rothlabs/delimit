use serde::Serialize;
use super::*;

pub type Ace<L> = Node<work::Ace<L>>;
pub type Deuce<U> = Node<work::Deuce<U>>;
pub type Trey<U, T, L> = Node<work::Trey<U, T, L>>;
pub type Agent<U> = Node<work::Agent<U>>;
pub type Pipe<U> = Node<work::Pipe<U>>;

/// A node creates an interactive bridge between root edges and work.
pub struct Node<W> {
    meta: Meta,
    ring: Ring,
    work: W,
}

impl<W> Default for Node<W>
where
    W: Default,
{
    fn default() -> Self {
        Self {
            meta: Meta::new(),
            ring: Ring::new(),
            work: W::default(),
        }
    }
}

impl<W> DoMake for Node<W>
where
    W: DoMake,
{
    type Unit = W::Unit;
    fn do_make<F: FnOnce(&Back) -> Self::Unit>(&mut self, make: F, back: &Back) {
        self.work.do_make(make, back);
    }
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

// impl<W> Make for Node<W>
// where
//     W: Dummy,
// {
//     type Item = W::Unit;
//     fn new<F: FnOnce(&Back) -> Self::Item>(new: F, back: &Back) -> Self {
//         Self {
//             meta: Meta::new(),
//             ring: Ring::new(),
//             work: W::set_unit(new, back),
//         }
//     }
// }

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

impl<W> DoWrite for Node<W>
where
    W: DoWrite,
{
    type Item = W::Item;
    fn do_write<F: FnOnce(&mut Self::Item)>(&mut self, write: F) {
        self.work.do_write(write);
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

impl<W> DoRead for Node<W>
where
    W: DoRead,
{
    type Item = W::Item;
    fn do_read(&self) -> &Self::Item {
        self.work.do_read()
    }
}

impl<W> DoGrant for Node<W>
where
    W: DoGrant,
{
    type Load = W::Load;
    fn do_grant(&mut self, back: &Back) -> Self::Load {
        self.work.do_grant(back)
    }
}

impl<W> DoAct for Node<W>
where
    W: DoAct,
{
    type Load = W::Load;
    fn do_act(&mut self, back: &Back) -> Self::Load {
        self.work.do_act(back)
    }
}

impl<W> DoSolve for Node<W>
where
    W: DoSolve,
{
    type Task = W::Task;
    type Load = W::Load;
    fn do_solve(&mut self, task: Self::Task) -> Self::Load {
        self.work.do_solve(task)
    }
}

impl<W> DoAddRoot for Node<W> {
    fn do_add_root(&mut self, root: Root) {
        self.ring.add_root(root);
    }
}

impl<W> DoUpdate for Node<W> where W: Clear + DoReact + Threading {}

impl<W> DoRebut for Node<W>
where
    W: Clear,
{
    fn do_rebut(&mut self) -> Ring {
        self.work.clear();
        self.ring.rebut()
    }
}

impl<W> DoReact for Node<W>
where
    W: DoReact,
{
    fn do_react(&mut self, meta: &Meta) {
        self.work.do_react(meta);
    }
}

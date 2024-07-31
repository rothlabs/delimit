use super::*;

pub type Ace<L> = Apex<work::Ace<L>>;
pub type Agent<U> = Apex<work::Agent<U>>;

/// A apex creates an interactive bridge between root edges and work.
pub struct Apex<W> {
    meta: Meta,
    ring: Ring,
    work: W,
}

impl<W> Default for Apex<W>
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

impl<W> FromItem for Apex<W>
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

impl<W> ToMeta for Apex<W> {
    fn meta(&self) -> Meta {
        self.meta.clone()
    }
}

impl<W> DoMake for Apex<W>
where
    W: DoMake,
{
    type Unit = W::Unit;
    fn do_make<F: FnOnce(&Back) -> Self::Unit>(&mut self, make: F, back: &Back) {
        self.work.do_make(make, back);
    }
}

// impl<W> ToSerial for Apex<W>
// where
//     W: Serialize,
// {
//     fn serial(&mut self, serial: &'static mut Serial) -> &mut Serial {
//         // TODO: need to call serial on work as well and put items in HashMap with key as ID!!!
//         serial.add(&self.work)
//     }
// }

impl<W> ToLoad for Apex<W>
where
    W: ToLoad,
{
    type Load = W::Load;
    fn load(&self) -> Self::Load {
        self.work.load()
    }
}

impl<W> WriteLoadOut for Apex<W>
where
    W: WriteLoadWork,
{
    type Item = W::Item;
    fn write_load_out<T, F: FnOnce(&mut Self::Item) -> T>(&mut self, write: F) -> write::Out<T> {
        let out = self.work.write_load_work(write);
        let (roots, meta) = self.ring.rebut_roots(&self.meta);
        write::Out { roots, meta, out }
    }
}

impl<W> WriteUnitOut for Apex<W>
where
    W: WriteUnitWork,
{
    type Unit = W::Unit;
    fn write_unit_out<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(
        &mut self,
        write: F,
        back: &Back,
    ) -> write::Out<T> {
        let out = self.work.write_unit_work(write, back);
        let (roots, meta) = self.ring.rebut_roots(&self.meta);
        write::Out { roots, meta, out }
    }
}

impl<W> DoRead for Apex<W>
where
    W: DoRead,
{
    type Item = W::Item;
    fn do_read(&self) -> &Self::Item {
        self.work.do_read()
    }
}

impl<W> DoGrant for Apex<W>
where
    W: DoGrant,
{
    type Load = W::Load;
    fn do_grant(&mut self, back: &Back) -> Self::Load {
        self.work.do_grant(back)
    }
}

impl<W> DoAddRoot for Apex<W> {
    fn do_add_root(&mut self, root: Root) {
        self.ring.add_root(root);
    }
}

impl<W> DoUpdate for Apex<W> where W: Clear + DoReact + SendSync {}

impl<W> DoRebut for Apex<W>
where
    W: Clear,
{
    fn do_rebut(&mut self) -> Ring {
        self.work.clear();
        self.ring.rebut()
    }
}

impl<W> DoReact for Apex<W>
where
    W: DoReact,
{
    fn do_react(&mut self, meta: &Meta) -> react::Result {
        self.work.do_react(meta)
    }
}

impl<W> InsertMut for Apex<W> 
where 
    W: InsertMut
{
    fn insert_mut(&mut self, field: &str, node: Node) {
        self.work.insert_mut(field, node);
    }
}

// impl<W> DoWrite for Apex<W>
// where
//     W: DoWrite,
// {
//     type Item = W::Item;
//     fn do_write<T, F: FnOnce(&mut Self::Item) -> T>(&mut self, write: F) -> T {
//         let out = self.work.do_write(write);
//         self.ring.cycle(&self.meta);
//         out
//     }
// }

// impl<W> WriteWithBack for Apex<W>
// where
//     W: WriteWithBack,
// {
//     type Unit = W::Unit;
//     fn write_with_back<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(
//         &mut self,
//         write: F,
//         back: &Back,
//     ) -> T {
//         let out = self.work.write_with_back(write, back);
//         self.ring.cycle(&self.meta);
//         out
//     }
// }

// impl<W> Make for Apex<W>
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

// impl<'a, W> From<&'a str> for Apex<W>
// where
//     &'a str: Into<W>
// {
//     fn from(value: &'a str) -> Self {
//         Self {
//             meta: Meta::new(),
//             ring: Ring::new(),
//             work: value.into(),
//         }
//     }
// }

// impl<W> From<String> for Apex<W>
// where
//     String: Into<W>
// {
//     fn from(value: String) -> Self {
//         Self {
//             meta: Meta::new(),
//             ring: Ring::new(),
//             work: value.into(),
//         }
//     }
// }

use super::*;

pub type Leaf = Apex<work::Leaf>;

pub type Agent<U> = Apex<work::Agent<U>>;

/// A apex creates an interactive bridge between root edges and work.
#[derive(Debug)]
pub struct Apex<W> {
    id: Id,
    ring: Ring,
    work: W,
}

impl<W> Default for Apex<W>
where
    W: Default,
{
    fn default() -> Self {
        Self {
            id: random(),
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
            id: random(),
            ring: Ring::new(),
            work: W::new(item),
        }
    }
}

impl<W> ToId for Apex<W> {
    fn id(&self) -> Id {
        self.id.clone()
    }
}

impl<W> MakeInner for Apex<W>
where
    W: MakeInner,
{
    type Unit = W::Unit;
    fn do_make<F: FnOnce(&Back) -> Self::Unit>(&mut self, make: F, back: &Back) {
        self.work.do_make(make, back);
    }
}

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
        let roots = self.ring.rebut_roots();
        write::Out { roots, out, id: self.id.clone() }
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
        let roots = self.ring.rebut_roots();
        write::Out { roots, out, id: self.id.clone() }
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

impl<W> DoReadLoad for Apex<W>
where
    W: DoReadLoad,
{
    fn do_read_load(&self) -> load::ResultRef {
        self.work.do_read_load()
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
    fn do_react(&mut self, id: &Id) -> react::Result {
        self.work.do_react(id)
    }
}

impl<W> DoSolve for Apex<W>
where
    W: DoSolve,
{
    fn do_solve(&mut self, task: Task, back: &Back) -> solve::Result {
        self.work.do_solve(task, back)
    }
}

impl<W> Adapt for Apex<W>
where
    W: Adapt,
{
    fn adapt(&mut self, post: Post) -> adapt::Result {
        self.work.adapt(post)
    }
}



// impl<W> Serialize for Apex<W> 
// where 
//     W: Serialize
// {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
//         self.work.serialize(serializer)
//     }
// }

// impl<W> SerializeGraphInner for Apex<W>
// where
//     W: Serialize + DoSolve,
// {
//     fn serial(&mut self, serial: &mut Serial, back: &Back) -> serial::Result {
//         serial.insert(&self.meta, serde_json::to_string(&self.work)?);
//         if let Tray::Nodes(nodes) = self.work.do_solve(Task::Stems, back)? {
//             for node in &nodes {
//                 node.serial(serial)?;
//             }
//         }
//         Ok(())
//     }
// }

// impl<W> ToMeta for Apex<W> {
//     fn meta(&self) -> Meta {
//         self.meta.clone()
//     }
// }



// impl<W> ToSerial for Apex<W>
// where
//     W: Serialize,
// {
//     fn serial(&mut self, serial: &'static mut Serial) -> &mut Serial {
//         // TODO: need to call serial on work as well and put items in HashMap with key as ID!!!
//         serial.add(&self.work)
//     }
// }

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

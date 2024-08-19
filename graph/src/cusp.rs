use super::*;

pub type Leaf = Cusp<work::Leaf>;

pub type Node<U> = Cusp<work::Node<U>>;

/// A cusp creates an interactive bridge between root edges and work.
#[derive(Debug)]
pub struct Cusp<W> {
    id: Id,
    ring: Ring,
    work: W,
    back: Option<Back>,
}

impl<W> Default for Cusp<W>
where
    W: Default,
{
    fn default() -> Self {
        Self {
            id: random(),
            ring: Ring::new(),
            work: W::default(),
            back: None,
        }
    }
}

impl<W> FromItem for Cusp<W>
where
    W: FromItem,
{
    type Item = W::Item;
    fn new(item: Self::Item) -> Self {
        Self {
            id: random(),
            ring: Ring::new(),
            work: W::new(item),
            back: None,
        }
    }
}

impl<W> ToId for Cusp<W> {
    fn id(&self) -> Id {
        self.id.clone()
    }
}

impl<W> MakeInner for Cusp<W>
where
    W: MakeInner,
{
    type Unit = W::Unit;
    fn do_make<F: FnOnce(&Back) -> Self::Unit>(&mut self, make: F, back: &Back) {
        self.back = Some(back.clone());
        self.work.do_make(make, back);
    }
}

impl<W> ToTray for Cusp<W>
where
    W: ToTray,
{
    type Tray = W::Tray;
    fn tray(&self) -> Self::Tray {
        self.work.tray()
    }
}

impl<W> WriteTrayOut for Cusp<W>
where
    W: WriteTrayWork,
{
    type Item = W::Item;
    fn write_tray_out<T, F: FnOnce(&mut Self::Item) -> T>(&mut self, write: F) -> write::Out<T> {
        let out = self.work.write_tray_work(write);
        let roots = self.ring.rebut_roots();
        write::Out {
            roots,
            out,
            id: self.id.clone(),
        }
    }
}

impl<W> WriteUnitOut for Cusp<W>
where
    W: WriteUnitWork,
{
    type Unit = W::Unit;
    fn write_unit_out<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(
        &mut self,
        write: F,
        //back: &Back,
    ) -> write::Out<T> {
        let out = self
            .work
            .write_unit_work(write, &self.back.clone().unwrap());
        let roots = self.ring.rebut_roots();
        write::Out {
            roots,
            out,
            id: self.id.clone(),
        }
    }
}

impl<W> DoRead for Cusp<W>
where
    W: DoRead,
{
    type Item = W::Item;
    fn do_read(&self) -> &Self::Item {
        self.work.do_read()
    }
}

impl<W> DoReadTray for Cusp<W>
where
    W: DoReadTray,
{
    fn do_read_tray(&self) -> tray::ResultRef {
        self.work.do_read_tray()
    }
}

impl<W> DoAddRoot for Cusp<W> {
    fn do_add_root(&mut self, root: Root) {
        self.ring.add_root(root);
    }
}

impl<W> DoUpdate for Cusp<W> where W: Clear + DoReact + SendSync {}

impl<W> DoRebut for Cusp<W>
where
    W: Clear,
{
    fn do_rebut(&mut self) -> Ring {
        self.work.clear();
        self.ring.rebut()
    }
}

impl<W> DoReact for Cusp<W>
where
    W: DoReact,
{
    fn do_react(&mut self, id: &Id) -> react::Result {
        self.work.do_react(id)
    }
}

impl<W> DoSolve for Cusp<W>
where
    W: DoSolve,
{
    fn do_solve(&mut self, task: Task) -> solve::Result {
        self.work.do_solve(task)
    }
}

impl<W> AdaptOut for Cusp<W>
where
    W: Adapt + Clear,
{
    fn adapt_out(&mut self, post: Post) -> write::Out<adapt::Result> {
        self.work.clear();
        let out = self.work
            .adapt(post.backed(self.back.as_ref().expect("No back in cusp adapt.")));
        let roots = self.ring.rebut_roots();
        write::Out {
            roots,
            out,
            id: self.id.clone(),
        }
    }
}

use super::*;

pub type Leaf = Cusp<work::Leaf>;

pub type Node<U> = Cusp<work::Node<U>>;

/// A cusp creates an interactive bridge between root edges and work nodes.
#[derive(Debug)]
pub struct Cusp<W> {
    id: Id,
    work: W,
    ring: Ring,
    back: Option<Back>,
}

impl<W> Default for Cusp<W>
where
    W: Default,
{
    fn default() -> Self {
        Self {
            id: rand::random(),
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
            id: rand::random(),
            ring: Ring::new(),
            work: W::new(item),
            back: None,
        }
    }
}

impl<W> ToId for Cusp<W> {
    fn id(&self) -> Id {
        self.id
    }
}

impl<W> MakeMid for Cusp<W>
where
    W: MakeMid,
{
    type Unit = W::Unit;
    fn make<F: FnOnce(&Back) -> Self::Unit>(&mut self, make: F, back: &Back) {
        self.back = Some(back.clone());
        self.work.make(make, back);
    }
}

impl<W> WithSnap for Cusp<W>
where
    W: WithSnap,
{
    type Unit = W::Unit;
    fn with_snap(&mut self, snap: Snap<Self::Unit>, back: &Back) {
        self.back = Some(back.clone());
        self.work.with_snap(snap, back);
    }
}

impl<W> WriteTrayOut for Cusp<W>
where
    W: WriteTrayWork,
{
    type Item = W::Item;
    fn write_tray_out<T, F: FnOnce(&mut Self::Item) -> T>(
        &mut self,
        write: F,
    ) -> Result<write::Out<T>> {
        let out = self.work.write_tray_work(write)?;
        let roots = self.ring.rebut_roots();
        Ok(write::Out {
            roots,
            out,
            id: self.id,
        })
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
    ) -> Result<write::Out<T>> {
        let out = self
            .work
            .write_unit_work(write, &self.back.clone().unwrap())?;
        let roots = self.ring.rebut_roots();
        Ok(write::Out {
            roots,
            out,
            id: self.id,
        })
    }
}

impl<W> ToItem for Cusp<W>
where
    W: ToItem,
{
    type Item = W::Item;
    fn item(&self) -> &Self::Item {
        self.work.item()
    }
}

impl<W> AddRootMut for Cusp<W> {
    fn add_root(&mut self, root: Root) {
        self.ring.add_root(root);
    }
}

impl<W> RebutMut for Cusp<W>
where
    W: Clear,
{
    fn rebut(&mut self) -> Ring {
        self.work.clear();
        self.ring.rebut()
    }
}

impl<W> ReactMut for Cusp<W>
where
    W: ReactMut,
{
    fn react(&mut self, id: &Id) -> react::Result {
        self.work.react(id)
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
    fn adapt(&mut self, post: Post) -> Result<write::Out<Memo>> {
        self.work.clear();
        let post = post.backed(self.back.as_ref().expect("No back in cusp adapt."))?;
        let out = self.work.adapt(post)?;
        let roots = self.ring.rebut_roots();
        Ok(write::Out {
            roots,
            out,
            id: self.id,
        })
    }
}

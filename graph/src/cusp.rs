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
        self.id.clone()
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

impl<W> FromSnapMid for Cusp<W>
where
    W: FromSnapMid,
{
    type Unit = W::Unit;
    fn from_snap(&mut self, snap: Snap<Self::Unit>, back: &Back) {
        self.back = Some(back.clone());
        self.work.from_snap(snap, back);
    }
}

impl<W> ToTray for Cusp<W>
where
    W: ToTray,
{
    fn tray(&self) -> Tray {
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

impl<W> ReadMid for Cusp<W>
where
    W: ReadMid,
{
    type Item = W::Item;
    fn read(&self) -> &Self::Item {
        self.work.read()
    }
}

impl<W> ReadTrayMid for Cusp<W>
where
    W: ReadTrayMid,
{
    fn read_tray(&self) -> tray::RefResult {
        self.work.read_tray()
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
    fn adapt(&mut self, post: Post) -> Result<write::Out<Memo>, crate::Error> {
        self.work.clear();
        let post = post.backed(self.back.as_ref().expect("No back in cusp adapt."))?;
        let out = self
            .work
            .adapt(post)?;
        let roots = self.ring.rebut_roots();
        Ok(write::Out {
            roots,
            out,
            id: self.id.clone(),
        })
    }
}

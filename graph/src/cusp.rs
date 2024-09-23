use super::*;
use anyhow::anyhow;

pub type Leaf<T> = Cusp<work::Leaf<T>>;

pub type Node<U> = Cusp<work::Node<U>>;

/// A cusp creates an interactive bridge between root edges and work nodes.
#[derive(Debug)]
pub struct Cusp<W> {
    work: W,
    ring: Ring,
    back: Option<Back>,
}

impl<W> FromBase for Cusp<W>
where
    W: 'static + WorkFromBase + Clear + ReactMut + SendSync,
{
    type Base = W::Base;
    fn from_base(base: W::Base) -> Pointer<Self> {
        let (cusp, _) = cusp_pointer(Self {
            work: W::from_base(base),
            ring: Ring::new(),
            back: None,
        });
        cusp
    }
}

impl<W> FromSnap for Cusp<W>
where
    W: 'static + WorkFromSnap + Clear + ReactMut + SolveMut + SendSync,
{
    type Unit = W::Unit;
    fn from_snap(snap: Snap<Self::Unit>) -> Result<(Option<u64>, Pointer<Self>)> {
        let (rank, work) = W::from_snap(snap);
        let (cusp, back) = cusp_pointer(Self {
            work,
            ring: Ring::new(),
            back: None,
        });
        write_part(&cusp, |mut cusp| cusp.set_back(back))??;
        Ok((rank, cusp))
    }
}

impl<W: SolveMut> Cusp<W> {
    fn set_back(&mut self, mut back: Back) -> Result<()> {
        if self.work.adapt(&mut back).is_err() {
            self.work.back(&back)?;
        }
        self.back = Some(back);
        Ok(())
    }
}

impl<W> WriteBaseOut for Cusp<W>
where
    W: BaseMut + Clear,
{
    type Base = W::Base;
    fn write_base_out<O, F: FnOnce(&mut W::Base) -> O>(&mut self, write: F) -> Result<(Ring, O)> {
        self.work.clear();
        let ring = self.ring.root_rebut()?;
        let out = write(self.work.base());
        Ok((ring, out))
    }
}

impl<W> WriteUnitOut for Cusp<W>
where
    W: WriteUnitWork + Clear,
{
    type Unit = W::Unit;
    fn write_unit_out<O, F>(&mut self, write: F) -> Result<(Ring, O)>
    where
        F: FnOnce(&mut Pack<Self::Unit>) -> O,
    {
        let back = self.back.as_ref().ok_or(anyhow!("no back"))?;
        self.work.clear();
        let ring = self.ring.root_rebut()?;
        let out = self.work.write_unit_work(write, back);
        Ok((ring, out))
    }
}

impl<W: ToItem> ToItem for Cusp<W> {
    type Item = W::Item;
    fn item(&self) -> &Self::Item {
        self.work.item()
    }
}

impl<W> AddRoot for Cusp<W> {
    fn add_root(&mut self, root: &Option<Root>) {
        if let Some(root) = root {
            self.ring.add_root(root.clone());
        }
    }
}

impl<W> RebutMut for Cusp<W>
where
    W: Clear,
{
    fn rebut(&mut self) -> Result<Ring> {
        self.work.clear();
        self.ring.rebut()
    }
    fn clear_roots(&mut self) -> Result<()> {
        self.ring.clear()
    }
}

impl<W> ReactMut for Cusp<W>
where
    W: ReactMut + SendSync,
{
    fn react_mut<'a>(&'a mut self) -> GraphFuture<Result<()>> {
        Box::pin(async move { self.work.react_mut().await })
    }
}

impl<W> SolveMut for Cusp<W>
where
    W: SolveMut + SendSync,
{
    type Base = W::Base;
    fn solve(&mut self) -> GraphFuture<Result<Hub<W::Base>>> {
        Box::pin(async move { self.work.solve().await })
    }
    fn reckon(&mut self, task: Task) -> Result<Gain> {
        self.work.reckon(task)
    }
}

impl<W> AdaptMut for Cusp<W>
where
    W: SolveMut + Clear,
{
    fn adapt_get(&mut self, deal: &mut dyn Deal) -> Result<()> {
        self.work.adapt(deal)
    }
    fn adapt_set(&mut self, deal: &mut dyn Deal) -> Result<Ring> {
        let back = self.back.as_ref().ok_or(anyhow!("no back"))?;
        deal.back(back);
        self.work.clear();
        self.work.adapt(deal)?;
        self.ring.root_rebut()
    }
}

#[cfg(not(feature = "oneThread"))]
pub fn cusp_pointer<T>(cusp: T) -> (Arc<RwLock<T>>, Back)
where
    T: 'static + UpdateMut,
{
    let cusp = Arc::new(RwLock::new(cusp));
    let update = cusp.clone() as Arc<RwLock<dyn UpdateMut>>;
    let back = Back {
        cusp: Arc::downgrade(&update),
        id: rand::random(),
    };
    (cusp, back)
}

#[cfg(feature = "oneThread")]
pub fn cusp_pointer<T>(cusp: T) -> (Rc<RefCell<T>>, Back)
where
    T: 'static + UpdateMut, // + SetBack,
{
    let cusp = Rc::new(RefCell::new(cusp));
    let update = cusp.clone() as Rc<RefCell<dyn UpdateMut>>;
    let back = Back {
        cusp: Rc::downgrade(&update),
        id: rand::random(),
    };
    (cusp, back)
}

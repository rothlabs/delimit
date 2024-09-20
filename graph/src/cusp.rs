use anyhow::anyhow;
use async_trait::async_trait;

use super::*;

pub type Leaf<T> = Cusp<work::Leaf<T>>;

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

impl<W: Adapt> SetBack for Cusp<W> {
    fn set_back(&mut self, mut back: Back) -> Result<()> {
        self.work.adapt(&mut back)?;
        self.back = Some(back);
        Ok(())
    }
}

impl<W> Make for Cusp<W> 
where 
    W: 'static + MakeWork + Default + Clear + ReactMut + Adapt + SendSync,
{
    type Unit = W::Unit;
    fn make(unit: Self::Unit) -> Result<(Option<u64>, Pointer<Self>)> {
        let (rank, work) = W::make(unit);
        let (cusp, back) = cusp_pointer(Self {
            work,
            ..Self::default()
        });
        cusp.write().set_back(back)?;
        Ok((rank, cusp))
    }
}

// impl<W> InitMut for Cusp<W>
// where
//     W: InitMut,
// {
//     type Unit = W::Unit;
//     fn init<F: FnOnce(&Back) -> Result<Self::Unit>>(
//         &mut self,
//         make: F,
//         back: &Back,
//     ) -> Result<Option<u64>> {
//         self.back = Some(back.clone());
//         self.work.init(make, back)
//     }
// }

impl<W> WithSnap for Cusp<W>
where
    W: WithSnap,
{
    type Unit = W::Unit;
    fn with_snap(&mut self, snap: Snap<Self::Unit>, back: &Back) -> Option<u64> {
        self.back = Some(back.clone());
        self.work.with_snap(snap, back)
    }
}

impl<W, T> WriteBaseOut<T> for Cusp<W>
where
    W: BaseMut<T>,
{
    fn write_base_out<O, F: FnOnce(&mut T) -> O>(&mut self, write: F) -> Result<Post<O>> {
        let out = write(self.work.base());
        let roots = self.ring.rebut_roots()?;
        Ok(Post {
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
    ) -> Result<Post<T>> {
        // TODO: remove unrwap
        let out = self
            .work
            .write_unit_work(write, &self.back.clone().unwrap())?;
        let roots = self.ring.rebut_roots()?;
        Ok(Post {
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
    fn rebut(&mut self) -> Result<Ring> {
        self.work.clear();
        self.ring.rebut()
    }
    fn clear_roots(&mut self) -> Result<()> {
        self.ring.clear()
    }
}

// impl<W> ClearRootsMut for Cusp<W> {
//     fn clear_roots(&mut self) -> Result<()> {
//         self.ring.clear()
//     }
// }

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<W> ReactMut for Cusp<W>
where
    W: ReactMut + SendSync,
{
    async fn react(&mut self, id: &Id) -> Result<()> {
        self.work.react(id).await
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<W> SolveMut for Cusp<W>
where
    W: SolveMut + SendSync,
    W::Base: Payload,
{
    type Base = W::Base;
    async fn solve(&mut self) -> Result<Hub<W::Base>> {
        self.work.solve().await
    }
}

impl<W> ReckonMut for Cusp<W>
where
    W: ReckonMut,
{
    fn reckon(&mut self, task: Task) -> Result<Gain> {
        self.work.reckon(task)
    }
}

impl<W> AdaptOut for Cusp<W>
where
    W: Adapt + Clear,
{
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<(Ring, u64)> {
        self.work.clear();
        if let Some(back) = self.back.as_ref() {
            deal.back(back);
        } else {
            return Err(anyhow!("No back in cusp adapt."))?;
        }
        self.work.adapt(deal)?;
        let roots = if deal.wrote() {
            self.ring.rebut_roots()?
        } else {
            Ring::new()
        };
        Ok((roots, self.id))
    }
}

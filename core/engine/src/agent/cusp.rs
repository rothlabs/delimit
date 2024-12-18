use super::*;

pub trait Solve {
    type Base;
    fn solve(&mut self) -> impl Future<Output = agent::Result<Self::Base>>;
    // fn back(&mut self, back: &Back);
}

pub trait Rebut {
    // fn rebut(&mut self) -> Result<Ring>;
    fn rebut(&self);
    // fn clear_roots(&mut self) -> Result<()>;
}

impl<T: super::Solve> Rebut for Cusp<T> {
    fn rebut(&self) {
        self.base.replace(None);
        // self.ring.rebut()
    }
    // fn clear_roots(&mut self) -> Result<()> {
    //     self.ring.clear()
    // }
}

pub trait React {
    /// Cause the unit to react. Call only on graph roots returned from the rebut phase.
    fn react(&self) -> PinFuture<crate::Result<()>> {
        Box::pin(async move { Ok(()) })
    }
}



// // #[derive(Clone, Debug)]
// pub struct Back {
//     pub cusp: Weak<dyn UpdateMut>,
//     // pub id: Id,
// }